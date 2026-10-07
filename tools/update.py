#!/usr/bin/env python3
"""Update an installed kit from a published release, without losing local work.

Why not "unzip the new version over the old one": a naive copy quietly destroys
three things.

  * Machine state.  agent-config.json is generated per machine by sc2.py init.
    A release never ships it, and an updater must never overwrite or remove it.
  * Local work.  A user's own files under the installed tree are not in the
    release, so "make the tree match the release" would delete them.
  * A mixed tree.  Copying file by file leaves old and new code interleaved if it
    stops halfway, and a half-updated kit is worse than a stale one.

So this plans against the release manifest, verifies every payload hash before it
writes anything, keeps a backup of each file it replaces, and rolls back on
failure.  It only ever adds or replaces.  It never deletes.

    python tools/update.py --check  dist/StarCraftIIAgent-0.2.0.zip
    python tools/update.py --apply  dist/StarCraftIIAgent-0.2.0.zip
    python tools/update.py --apply  https://example.invalid/kit-0.2.0.zip
    python tools/update.py --check  ./unpacked-release/     # a directory works too

Exit codes: 0 = up to date or applied, 1 = a problem, 2 = bad usage.
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import sys
import tempfile
import urllib.error
import urllib.request
import zipfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import sc2_console  # noqa: E402
import sc2_version  # noqa: E402
from sc2_util import digest_bytes, digest_file  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
SELF_ENTRY = ".sc2-manifest.json"
VERSION_ENTRY = "VERSION"
BACKUP_DIRNAME = ".sc2-update-backup"
# Never written by an update, whatever a manifest claims.
NEVER_TOUCH = {"agent-config.json"}
DOWNLOAD_TIMEOUT = 60


class UpdateError(RuntimeError):
    """Something made the update unsafe; nothing has been written."""


def safe_rel(rel: str) -> str:
    """Reject any manifest path that would write outside the kit.

    The manifest is data from someone else, so the paths in it are not trusted:
    an absolute path or a ".." segment would let a release write anywhere the
    user can.
    """
    text = rel.strip().replace("\\", "/")
    if not text or text.startswith("/") or Path(text).is_absolute():
        raise UpdateError(f"发布清单里的路径不安全：{rel!r}")
    parts = [p for p in text.split("/") if p not in ("", ".")]
    if any(p == ".." for p in parts):
        raise UpdateError(f"发布清单里的路径试图跳出安装目录：{rel!r}")
    if parts and parts[0] in NEVER_TOUCH:
        raise UpdateError(f"发布清单试图写入本机专属文件：{rel!r}")
    return "/".join(parts)


class Release:
    """A published release and a way to read its payloads.

    Owns whatever resource backs the payloads (an open zip, usually) so the caller
    can release it.  Without that, an updater that read from a zip on Windows left
    the file locked and the user could not delete the download afterwards.
    """

    def __init__(self, label: str, manifest: dict, reader, closer=None,
                 raw_manifest: bytes | None = None):
        self.label = label
        self.manifest = manifest
        self.reader = reader
        self._closer = closer
        self.raw_manifest = raw_manifest

    def close(self) -> None:
        closer, self._closer = self._closer, None
        if closer is not None:
            try:
                closer()
            except OSError:
                pass

    def __enter__(self) -> "Release":
        return self

    def __exit__(self, *exc) -> bool:
        self.close()
        return False

    @property
    def version(self) -> str:
        return str(self.manifest.get("kit_version", "?"))

    @property
    def layout_revision(self) -> int:
        try:
            return int(self.manifest.get("layout_revision", 0))
        except (TypeError, ValueError):
            return 0

    def payload(self, rel: str) -> bytes:
        return self.reader(rel)

    def checked_files(self) -> dict:
        files = self.manifest.get("files")
        if not isinstance(files, dict) or not files:
            raise UpdateError("发布清单里没有 files 段，可能是清单损坏或不是本工具的清单")
        return files


def release_from_zip(path: Path) -> Release:
    zf = zipfile.ZipFile(path)
    names = zf.namelist()
    entry = next((n for n in names if n.endswith("/" + SELF_ENTRY) or n == SELF_ENTRY), None)
    if entry is None:
        zf.close()
        raise UpdateError(f"{path} 里没有 {SELF_ENTRY}，不是本工具打出的发行包")
    prefix = entry[: -len(SELF_ENTRY)]
    raw = zf.read(entry)
    manifest = json.loads(raw.decode("utf-8"))

    def reader(rel: str) -> bytes:
        try:
            return zf.read(prefix + rel)
        except KeyError as exc:
            raise UpdateError(f"发行包里缺少清单列出的文件：{rel}") from exc

    return Release(str(path), manifest, reader, closer=zf.close, raw_manifest=raw)


def release_from_directory(path: Path) -> Release:
    candidates = [path / SELF_ENTRY] + sorted(path.glob("*.manifest.json"))
    entry = next((p for p in candidates if p.is_file()), None)
    if entry is None:
        raise UpdateError(f"{path} 里没有 {SELF_ENTRY} 或 *.manifest.json")
    raw = entry.read_bytes()
    manifest = json.loads(raw.decode("utf-8"))

    def reader(rel: str) -> bytes:
        target = path / rel
        if not target.is_file():
            raise UpdateError(f"发布目录里缺少清单列出的文件：{rel}")
        return target.read_bytes()

    return Release(str(path), manifest, reader, raw_manifest=raw)


def download(url: str, into: Path) -> Path:
    target = into / "release.zip"
    try:
        with urllib.request.urlopen(url, timeout=DOWNLOAD_TIMEOUT) as response:
            target.write_bytes(response.read())
    except (urllib.error.URLError, OSError, ValueError) as exc:
        raise UpdateError(f"下载失败：{url}（{exc}）") from exc
    return target


def load_release(source: str, workdir: Path) -> Release:
    if source.startswith(("http://", "https://")):
        return release_from_zip(download(source, workdir))
    path = Path(source).expanduser()
    if path.is_dir():
        return release_from_directory(path)
    if path.is_file():
        return release_from_zip(path)
    raise UpdateError(f"找不到发布来源：{source}")


def installed_version(root: Path) -> str:
    version_file = root / VERSION_ENTRY
    if version_file.is_file():
        text = version_file.read_text(encoding="utf-8", errors="replace").strip()
        if text:
            return text
    return sc2_version.VERSION


def build_plan(release: Release, root: Path) -> dict:
    """Classify every listed file as added, changed or current, without writing."""
    added, changed, current = [], [], []
    for rel, entry in sorted(release.checked_files().items()):
        rel = safe_rel(rel)
        target = root / rel
        expected = entry.get("sha256")
        if not target.is_file():
            added.append(rel)
        elif expected and digest_file(target) != expected:
            changed.append(rel)
        else:
            current.append(rel)
    listed = {safe_rel(r) for r in release.checked_files()}
    local_only = []
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(root).as_posix()
        if rel in listed or rel in (VERSION_ENTRY, SELF_ENTRY):
            continue
        if BACKUP_DIRNAME in path.parts or rel.split("/")[0].startswith(".sc2-update-stage-"):
            continue
        local_only.append(rel)
    return {"added": added, "changed": changed, "current": current, "local_only": local_only}


def verify_payloads(release: Release, plan: dict) -> None:
    """Hash every payload before any of them is written.

    Verifying as we go would discover a corrupt payload halfway through the
    update, which is the one outcome the backup exists to avoid.
    """
    for rel in plan["added"] + plan["changed"]:
        payload = release.payload(rel)
        expected = release.checked_files()[rel].get("sha256")
        if expected and digest_bytes(payload) != expected:
            raise UpdateError(f"校验失败：{rel} 的内容与清单里的 SHA-256 不一致")


def apply_plan(release: Release, plan: dict, root: Path) -> Path:
    """Write the plan, keeping a backup and rolling back if anything fails."""
    stamp = f"{installed_version(root)}-to-{release.version}"
    backup = root / BACKUP_DIRNAME / stamp
    backup.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=".sc2-update-stage-", dir=str(root)))
    journal: list[tuple[str, str]] = []   # (kind, rel): "replace" or "add"

    try:
        for rel in plan["added"] + plan["changed"]:
            staged = stage / rel
            staged.parent.mkdir(parents=True, exist_ok=True)
            staged.write_bytes(release.payload(rel))

        for rel in plan["added"] + plan["changed"]:
            target = root / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            if target.is_file():
                saved = backup / rel
                saved.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(target, saved)
                journal.append(("replace", rel))
            else:
                journal.append(("add", rel))
            os.replace(stage / rel, target)

        (root / VERSION_ENTRY).write_text(release.version + "\n", encoding="utf-8", newline="\n")
        # Refresh the installed manifest too.  Leaving the previous release's copy
        # behind would leave the tree carrying a manifest that describes a
        # different tree than the one on disk.
        if release.raw_manifest is not None:
            (root / SELF_ENTRY).write_bytes(release.raw_manifest)
    except Exception as original:
        restored, failed = 0, []
        for kind, rel in reversed(journal):
            try:
                target = root / rel
                if kind == "replace":
                    shutil.copy2(backup / rel, target)
                elif target.exists():
                    target.unlink()
                restored += 1
            except OSError:
                failed.append(rel)
        detail = f"已回滚 {restored} 个文件"
        if failed:
            detail += f"，但 {len(failed)} 个未能回滚：{failed[:5]}（备份在 {backup}）"
        raise UpdateError(f"更新失败并{detail}。原因：{original}") from original
    finally:
        shutil.rmtree(stage, ignore_errors=True)

    return backup


def report_plan(release: Release, plan: dict, root: Path, as_json: bool) -> None:
    if as_json:
        print(json.dumps({
            "release": release.label, "release_version": release.version,
            "installed_version": installed_version(root),
            "added": len(plan["added"]), "changed": len(plan["changed"]),
            "current": len(plan["current"]), "local_only": len(plan["local_only"]),
        }, ensure_ascii=False))
        return
    print(f"发行包  : {release.label}")
    print(f"版本    : 已装 {installed_version(root)}  ->  发行 {release.version}")
    print(f"新增 {len(plan['added'])}   更新 {len(plan['changed'])}   "
          f"已是最新 {len(plan['current'])}   本机额外文件 {len(plan['local_only'])}")
    if plan["added"]:
        print("\n将新增：")
        for rel in plan["added"][:12]:
            print(f"  + {rel}")
        if len(plan["added"]) > 12:
            print(f"  ... 还有 {len(plan['added']) - 12} 个")
    if plan["changed"]:
        print("\n将更新：")
        for rel in plan["changed"][:12]:
            print(f"  ~ {rel}")
        if len(plan["changed"]) > 12:
            print(f"  ... 还有 {len(plan['changed']) - 12} 个")
    if plan["local_only"]:
        print(f"\n本机额外文件 {len(plan['local_only'])} 个：更新不会动它们，"
              "也不会删除（agent-config.json 属此类）。")


def main() -> int:
    sc2_console.enable_utf8_output()

    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("source", nargs="?", help="发行包 zip、解压目录，或 http(s) 地址")
    parser.add_argument("--check", action="store_true", help="只报告差异（默认行为）")
    parser.add_argument("--apply", action="store_true", help="真正写入更新")
    parser.add_argument("--root", default=str(ROOT), help="要更新的安装目录（默认：本 kit）")
    parser.add_argument("--force", action="store_true", help="版本不更新也照样应用")
    parser.add_argument("--json", action="store_true", help="机器可读输出")
    parser.add_argument("--list-backups", action="store_true", help="列出已保留的备份")
    args = parser.parse_args()

    root = Path(args.root).expanduser().resolve()

    if args.list_backups:
        base = root / BACKUP_DIRNAME
        if not base.is_dir():
            print("没有备份。")
            return 0
        for entry in sorted(base.iterdir()):
            count = sum(1 for p in entry.rglob("*") if p.is_file())
            print(f"  {entry.name}  ({count} 个文件)")
        return 0

    if not args.source:
        parser.print_help()
        return 2
    if not root.is_dir():
        print(f"安装目录不存在：{root}", file=sys.stderr)
        return 2

    try:
        with tempfile.TemporaryDirectory(prefix="sc2-update-") as tmp, \
                load_release(args.source, Path(tmp)) as release:
            plan = build_plan(release, root)

            if release.layout_revision > sc2_version.LAYOUT_REVISION:
                raise UpdateError(
                    "发行包的目录布局版本更新（layout_revision "
                    f"{release.layout_revision} > 本机 {sc2_version.LAYOUT_REVISION}），"
                    "无法就地更新。请解压完整发行包覆盖安装。"
                )

            if not args.apply:
                report_plan(release, plan, root, args.json)
                if not plan["added"] and not plan["changed"]:
                    print("\n已经是最新，无需更新。")
                else:
                    print("\n这是预演。加 --apply 才会写入。")
                return 0

            if not plan["added"] and not plan["changed"] and not args.force:
                if args.json:
                    print(json.dumps({"status": "up-to-date",
                                      "version": installed_version(root)}, ensure_ascii=False))
                else:
                    print("已经是最新，无需更新。")
                return 0

            if not plan["added"] and not plan["changed"] and args.force:
                print("没有文件需要写入（--force 也不会重写已是最新的文件）。")
                return 0

            verify_payloads(release, plan)
            backup = apply_plan(release, plan, root)
    except UpdateError as exc:
        print(f"更新中止：{exc}", file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps({"status": "applied", "version": release.version,
                          "added": len(plan["added"]), "changed": len(plan["changed"]),
                          "backup": str(backup)}, ensure_ascii=False))
        return 0

    print(f"已更新到 {release.version}：新增 {len(plan['added'])}，更新 {len(plan['changed'])}。")
    print(f"备份保留在 {backup}")
    print("回退方法：把备份目录里的文件按相同相对路径复制回去。")
    print("\n下一步：")
    print("  python tools/sc2.py            # 确认前端可用")
    print("  python tools/test-suite.py --scope tools    # 跑一遍单测")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
