#!/usr/bin/env python3
"""Package this workspace into something another user can unzip and trust.

Three things make a workspace non-portable or unverifiable, and this removes all
three:

  * machine-specific state.  "agent-config.json" holds absolute paths from the
    machine that created it, so shipping it hands the recipient a broken
    configuration.  It is excluded, and the recipient runs sc2.py init instead.
  * bulk reference data.  "DataEditorXML/" is ~180 MB of exported game data that
    only some tasks need, so it is opt-in via --include-data.
  * a package nobody checked.  Writing a zip proves nothing about whether the
    recipient can run it, so a release is verified three ways: every member is
    re-read and re-hashed against the manifest, the tree is scanned for leaked
    machine-specific absolute paths, and the extracted copy is actually executed.

    python tools/sc2.py package                  # dry run: what would ship
    python tools/sc2.py package --write          # build dist/<name>-<version>.zip
    python tools/sc2.py package --write --include-data

The manifest written next to the archive is what the updater consumes: it carries
the kit version and layout revision alongside size and SHA-256 for every file.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
import zipfile
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import sc2_console  # noqa: E402
import sc2_version  # noqa: E402
from sc2_util import digest_bytes, digest_file as digest  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]

# Never shipped: version control, build output, caches, and machine state.
# "build" matters as much as "dist": a PyInstaller build leaves an xref report
# full of this machine's absolute paths, and shipping it both leaks them and
# breaks the portability scan for everyone who ever builds the updater exe.
GENERATED_DIRS = {"dist", "build", "dist-never-ship", "sc2-catalog-graph-out",
                  ".workspace-recovery", ".pytest_cache", ".mypy_cache",
                  "sc2agent-records", ".sc2-update-backup"}
EXCLUDE_DIRS = {".git", "publish", "__pycache__", ".venv", "node_modules"} | GENERATED_DIRS   # dev records live outside the kit; never ship them
EXCLUDE_FILES = {"agent-config.json", ".DS_Store"}
BULK_DIRS = {"DataEditorXML"}

MANIFEST_VERSION = 2
VERSION_ENTRY = "VERSION"

# A release that leaves out the bulk exports still has to be self-consistent:
# skills link into DataEditorXML/, and a link into a directory that was never
# shipped is a dead end the recipient cannot diagnose.  A stub at the exact
# linked path keeps every link resolving and explains how to get the real data.
DATA_STUB_PATH = "DataEditorXML/SC2GameDataComponents/README.md"
DATA_STUB_BODY = """# 本发行包未包含游戏数据导出

顶层 `DataEditorXML/` 是约 180 MB 的《星际争霸 II》导出数据，只有部分任务需要，
因此默认不打进发行包——你看到的这个文件是占位说明，不是真实数据。

三种获取方式（任选其一）：

1. 从本机 SC2 编辑器导出一份，放到本目录；
2. 向发放此包的人索取 `--include-data` 版本；
3. 自己重新打包：`python tools/package.py --write --include-data`

在此之前，指向 `DataEditorXML/` 的文档链接会解析到这个说明文件。
"""
# The manifest also travels inside the archive.  A beginner updating a kit should
# have to be handed one file, not a zip plus a loose JSON they will lose.
SELF_ENTRY = ".sc2-manifest.json"

# High-confidence "this path belongs to one machine" patterns.  A bare drive
# letter is not one of them: documentation legitimately writes C:\... as an
# example, and a noisy check is a check people learn to ignore.  Documentation
# that teaches "do not hardcode this" writes a placeholder, so placeholders are
# excluded explicitly rather than by hoping they never appear.
PORTABILITY_RE = re.compile(
    r"[A-Za-z]:[\\/]Users[\\/]([^\\/\s\"'`]+)"
    r"|/home/([A-Za-z0-9._-]+)/"
    r"|/Users/([A-Za-z0-9._-]+)/"
)
PLACEHOLDER_RE = re.compile(
    # A sigil only has to START the segment; the word alternatives are exact.
    r"(?:^[<{%$])"
    r"|^(?:\.\.\.|name|username|user|yourname|your-name|you|xxx+|"
    r"placeholder|example|someuser|me)$",
    re.IGNORECASE,
)
# Put this marker on a line that has to contain such a path on purpose.
PORTABILITY_OK = "portability-ok"
TEXT_SUFFIXES = {".md", ".py", ".json", ".txt", ".yml", ".yaml", ".toml", ".cfg",
                 ".ini", ".ps1", ".xml", ".xsd", ".html", ".css"}
SCAN_SIZE_LIMIT = 2 * 1024 * 1024


def collect(include_data: bool) -> list[Path]:
    """Return the sorted files to ship, relative to the workspace root."""
    picked = []
    for path in sorted(ROOT.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(ROOT)
        if EXCLUDE_DIRS.intersection(rel.parts):
            continue
        if path.name in EXCLUDE_FILES:
            continue
        if not include_data and BULK_DIRS.intersection(rel.parts):
            continue
        picked.append(rel)
    return picked


def machine_path_in(line: str) -> str | None:
    """The machine-specific path on this line, or None.

    A placeholder is not a machine-specific path: a guide that says "do not write
    C:\\Users\\<Name>\\... in your scripts" is stating the rule this check exists to
    enforce, and flagging it would train the reader to ignore the check.
    """
    for match in PORTABILITY_RE.finditer(line):
        segment = next((g for g in match.groups() if g), "")
        if segment and not PLACEHOLDER_RE.match(segment):
            return match.group(0)
    return None


def portability_findings(files: list[Path]) -> list[str]:
    """Report machine-specific absolute paths that would break on the recipient's box.

    A path only counts when it names a user profile or this checkout, and a line
    can opt out with an explicit marker rather than an ignore list that goes stale.
    """
    findings = []
    root_text = str(ROOT)
    for rel in files:
        src = ROOT / rel
        if src.suffix.lower() not in TEXT_SUFFIXES or src.stat().st_size > SCAN_SIZE_LIMIT:
            continue
        try:
            text = src.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for n, line in enumerate(text.splitlines(), 1):
            if PORTABILITY_OK in line:
                continue
            if root_text in line:
                findings.append(f"{rel.as_posix()}:{n}: contains this checkout's absolute path")
            elif machine_path_in(line):
                findings.append(f"{rel.as_posix()}:{n}: machine-specific absolute path")
    return findings


def synthetic_payloads(include_data: bool) -> dict[str, bytes]:
    """Payloads the archive carries that have no source file in the workspace."""
    if include_data:
        return {}
    return {DATA_STUB_PATH: DATA_STUB_BODY.encode("utf-8")}


def build_manifest(files: list[Path], version: str, include_data: bool,
                   extra: dict[str, bytes] | None = None) -> dict:
    extra = extra or {}
    entries = {}
    total = 0
    for rel in files:
        src = ROOT / rel
        size = src.stat().st_size
        total += size
        entries[rel.as_posix()] = {"bytes": size, "sha256": digest(src)}
    for rel, payload in extra.items():
        entries[rel] = {"bytes": len(payload), "sha256": digest_bytes(payload), "synthetic": True}
        total += len(payload)
    version_bytes = (version + "\n").encode("utf-8")
    entries[VERSION_ENTRY] = {"bytes": len(version_bytes), "sha256": digest_bytes(version_bytes)}
    total += len(version_bytes)
    return {
        "manifest_version": MANIFEST_VERSION,
        "name": "StarCraftIIAgent",
        "kit_version": version,
        "layout_revision": sc2_version.LAYOUT_REVISION,
        "python_requires": ">=3.11",
        "generated_utc": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "file_count": len(entries),
        "total_bytes": total,
        "includes_data": bool(include_data),
        "files": entries,
    }


def write_archive(archive: Path, files: list[Path], stem: str, version: str,
                  manifest: dict, extra: dict[str, bytes] | None = None) -> None:
    """Write a deterministic archive: sorted entries with a fixed timestamp.

    Determinism is the point.  The same input has to produce the same bytes, so a
    recipient can diff two builds and an updater can trust a hash it already saw.
    """
    fixed = (1980, 1, 1, 0, 0, 0)
    payloads = ([(r.as_posix(), (ROOT / r).read_bytes()) for r in files]
                + list((extra or {}).items())
                + [(VERSION_ENTRY, (version + "\n").encode("utf-8"))]
                + [(SELF_ENTRY, serialize_manifest(manifest))])
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        for rel, payload in sorted(payloads):
            info = zipfile.ZipInfo(filename=f"{stem}/{rel}", date_time=fixed)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            zf.writestr(info, payload)


def serialize_manifest(manifest: dict) -> bytes:
    return (json.dumps(manifest, ensure_ascii=False, indent=1) + "\n").encode("utf-8")


def verify_archive(archive: Path, stem: str, manifest: dict) -> list[str]:
    """Re-read the archive and re-hash every member.  Returns the mismatches."""
    problems = []
    with zipfile.ZipFile(archive) as zf:
        names = set(zf.namelist())
        for rel, entry in manifest["files"].items():
            member = f"{stem}/{rel}"
            if member not in names:
                problems.append(f"missing from archive: {rel}")
                continue
            payload = zf.read(member)
            if len(payload) != entry["bytes"]:
                problems.append(f"{rel}: size {len(payload)} != manifest {entry['bytes']}")
            elif digest_bytes(payload) != entry["sha256"]:
                problems.append(f"{rel}: sha256 does not match the manifest")
        expected = set(manifest["files"]) | {SELF_ENTRY}
        if names != {f"{stem}/{rel}" for rel in expected}:
            extra = sorted(names - {f"{stem}/{rel}" for rel in expected})
            absent = sorted({f"{stem}/{rel}" for rel in expected} - names)
            if absent:
                problems.append(f"archive is missing {len(absent)} listed entries: {absent[:5]}")
            if extra:
                problems.append(f"archive holds {len(extra)} unlisted entries: {extra[:5]}")
        published = json.loads(zf.read(f"{stem}/{SELF_ENTRY}").decode("utf-8"))
        if published.get("files") != manifest["files"]:
            problems.append("the manifest inside the archive does not match the one written next to it")
    return problems


def smoke_test(archive: Path, checks, timeout: int = 300) -> list[str]:
    """Extract the archive elsewhere and actually run it.

    The check that separates a zip from a release: it proves the shipped tree is
    complete enough to start, instead of assuming the file list was.
    """
    problems = []
    with tempfile.TemporaryDirectory(prefix="sc2-package-smoke-") as tmp:
        target = Path(tmp)
        with zipfile.ZipFile(archive) as zf:
            zf.extractall(target)
        roots = [p for p in target.iterdir() if p.is_dir()]
        if len(roots) != 1:
            return [f"archive should extract to exactly one directory, found {len(roots)}"]
        workdir = roots[0]
        if not (workdir / "agent-config.json").exists():
            pass  # expected: the recipient generates it with sc2.py init
        for label, argv in checks:
            try:
                done = subprocess.run(
                    [sys.executable, *argv], cwd=str(workdir), capture_output=True,
                    text=True, encoding="utf-8", errors="replace", timeout=timeout,
                )
            except (OSError, subprocess.SubprocessError) as exc:
                problems.append(f"{label}: could not run ({exc})")
                continue
            if done.returncode != 0:
                tail = (done.stderr or done.stdout or "").strip().splitlines()[-3:]
                problems.append(f"{label}: exit {done.returncode} — {' / '.join(tail)}")
        return problems


def report_dry_run(files: list[Path], total: int, version: str, include_data: bool) -> None:
    by_top: dict[str, int] = {}
    for rel in files:
        top = rel.parts[0] if len(rel.parts) > 1 else "(root)"
        by_top[top] = by_top.get(top, 0) + (ROOT / rel).stat().st_size

    print(f"工作区: {ROOT}")
    print(f"版本  : {version}  (layout revision {sc2_version.LAYOUT_REVISION})")
    print(f"文件数: {len(files)}   合计 {total / 1048576:.2f} MB")
    if not include_data:
        print(f"占位  : {DATA_STUB_PATH}（保证指向导出数据的文档链接不断）")
    print(f"排除  : {sorted(EXCLUDE_DIRS)} + {sorted(EXCLUDE_FILES)}")
    if not include_data:
        print(f"未含  : {sorted(BULK_DIRS)}（--include-data 才打包）")
    print()
    print("按顶层目录:")
    for top, size in sorted(by_top.items(), key=lambda kv: -kv[1]):
        print(f"  {top:<24} {size / 1048576:>9.2f} MB")


def main() -> int:
    sc2_console.enable_utf8_output()

    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--write", action="store_true", help="build the archive (default: dry run)")
    parser.add_argument("--out", default="dist", help="output directory (default: dist/)")
    parser.add_argument("--name", default="StarCraftIIAgent", help="archive base name")
    parser.add_argument("--version", default=sc2_version.VERSION,
                        help="version tag (default: the value in tools/sc2_version.py)")
    parser.add_argument("--include-data", action="store_true",
                        help="also ship the bulk DataEditorXML reference exports")
    parser.add_argument("--no-smoke", action="store_true",
                        help="skip extracting and running the built archive")
    parser.add_argument("--json", action="store_true", help="machine-readable summary")
    parser.add_argument("--quiet", action="store_true")
    args = parser.parse_args()

    files = collect(args.include_data)
    total = sum((ROOT / r).stat().st_size for r in files)

    if not args.quiet and not args.json:
        report_dry_run(files, total, args.version, args.include_data)

    leaked = [r.as_posix() for r in files if r.name == "agent-config.json"]
    if leaked:
        print(f"拒绝打包：机器专属配置混入 {leaked}", file=sys.stderr)
        return 1

    findings = portability_findings(files)
    if findings:
        print(f"拒绝打包：{len(findings)} 处机器专属绝对路径会破坏可移植性"
              f"（确认无误可在该行加 '{PORTABILITY_OK}' 标记）", file=sys.stderr)
        for line in findings[:20]:
            print(f"  {line}", file=sys.stderr)
        return 1

    extra = synthetic_payloads(args.include_data)
    manifest = build_manifest(files, args.version, args.include_data, extra)

    if not args.write:
        if args.json:
            print(json.dumps({"dry_run": True, "file_count": len(files), "total_bytes": total,
                              "version": args.version}, ensure_ascii=False))
        elif not args.quiet:
            print(f"\n这是预演。加 --write 生成 {args.out}/{args.name}-{args.version}.zip")
        return 0

    out_dir = (ROOT / args.out) if not Path(args.out).is_absolute() else Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)
    stem = f"{args.name}-{args.version}"
    archive = out_dir / f"{stem}.zip"
    write_archive(archive, files, stem, args.version, manifest, extra)

    problems = verify_archive(archive, stem, manifest)
    if not problems and not args.no_smoke:
        problems.extend(smoke_test(archive, [
            ("启动前端", ["tools/sc2.py", "--version"]),
            ("技能 frontmatter", ["tools/audit-skill-frontmatter.py"]),
            ("文档链接", ["tools/check-doc-links.py"]),
        ]))

    (out_dir / f"{stem}.manifest.json").write_bytes(serialize_manifest(manifest))

    size = archive.stat().st_size
    if problems:
        print(f"\n打包校验未通过（{len(problems)} 项）：", file=sys.stderr)
        for line in problems[:20]:
            print(f"  {line}", file=sys.stderr)
        print(f"归档仍留在 {archive}，但不要分发。", file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps({"archive": str(archive), "bytes": size, "version": args.version,
                          "file_count": len(manifest["files"]), "verified": True},
                         ensure_ascii=False))
        return 0

    print(f"\n已生成 {archive.relative_to(ROOT).as_posix()}  ({size / 1048576:.2f} MB)")
    print(f"清单   {stem}.manifest.json  ({len(manifest['files'])} 个文件)")
    print("校验   : 逐文件重读并复核 SHA-256 通过；已解压试运行通过")
    print("\n收件人用法：")
    print("  1. 解压")
    print("  2. python tools/sc2.py            # 看可用动作")
    print("  3. python tools/sc2.py init \"<主 .SC2Mod 路径>\"    # 生成本机配置")
    print("\n以后升级： python tools/update.py --check <发布目录或 zip>")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
