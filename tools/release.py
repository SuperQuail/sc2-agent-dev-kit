#!/usr/bin/env python3
"""Build the release artifacts: a small core pack, a separate bulk-data pack, and a manifest.

Why the split: DataEditorXML is ~180 MB of exported game data that dwarfs everything
else in the kit.  Shipping it inside the core archive forces every recipient to
download and re-download it on every update.  Splitting it lets an installer compare
its hash and skip it entirely when nothing changed.

    python tools/release.py --write            # build dist/release/
    python tools/release.py                    # dry run

Layout written to dist/release/<version>/:
    StarCraftIIAgent-<version>-core.zip      kit minus the bulk data
    StarCraftIIAgent-<version>-data.zip      the bulk data (optional to install)
    StarCraftIIAgent-<version>-manifest.json every file's SHA-256, both artifact hashes
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
import zipfile
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sc2_version import VERSION, LAYOUT_REVISION  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]

EXCLUDE_DIRS = {".git", "publish", "__pycache__", ".venv", "node_modules",
                ".workspace-recovery", ".pytest_cache", ".mypy_cache", "sc2agent-records",
                "build",          # PyInstaller intermediate output
                "target",         # cargo build output; ~2 GB under installer/target
                "sc2-catalog-graph-out"}
# Build output is matched by prefix, not by exact name.  This kit keeps growing new
# ones (dist, dist-exe, dist-portable, ...) and an exact-name set silently ships
# whatever was added last.  Both inflation bugs so far were exactly this shape.
EXCLUDE_PREFIXES = ("dist",)
EXCLUDE_FILES = {"agent-config.json", ".DS_Store", "Thumbs.db"}
BULK_DIRS = {"DataEditorXML"}
FIXED_TIME = (1980, 1, 1, 0, 0, 0)
RELEASE_JSON = "sc2agent-release.json"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def collect(bulk: bool):
    picked = []
    for path in sorted(ROOT.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(ROOT)
        parts = rel.parts
        if set(parts) & EXCLUDE_DIRS:
            continue
        if any(p.startswith(EXCLUDE_PREFIXES) for p in parts[:-1]):
            continue
        if any(("installer/node_modules" in "/".join(parts[i:i + 2])) for i in range(len(parts))):
            continue
        if path.name in EXCLUDE_FILES:
            continue
        in_bulk = bool(BULK_DIRS.intersection(parts))
        if in_bulk != bulk:
            continue
        picked.append(rel)
    return picked


def write_zip(target: Path, files, extra: dict | None = None):
    with zipfile.ZipFile(target, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        for rel in files:
            info = zipfile.ZipInfo(filename=rel.as_posix(), date_time=FIXED_TIME)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            zf.writestr(info, (ROOT / rel).read_bytes())
        if extra:
            # A lone core.zip pulled straight from a GitHub release must describe
            # itself: the outer manifest lives beside it and is easy to miss.
            info = zipfile.ZipInfo(filename=RELEASE_JSON, date_time=FIXED_TIME)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o644 << 16
            zf.writestr(info, json.dumps(extra, ensure_ascii=False, indent=1) + "\n")


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--write", action="store_true", help="build the artifacts (default: dry run)")
    ap.add_argument("--out", default="dist/release")
    ap.add_argument("--skip-data", action="store_true", help="omit the bulk-data pack")
    args = ap.parse_args()

    core_files = collect(bulk=False)
    data_files = collect(bulk=True)
    core_bytes = sum((ROOT / r).stat().st_size for r in core_files)
    data_bytes = sum((ROOT / r).stat().st_size for r in data_files)

    print(f"version {VERSION}  layout_revision {LAYOUT_REVISION}")
    print(f"  core  {len(core_files):5d} files  {core_bytes / 1048576:9.2f} MB")
    print(f"  data  {len(data_files):5d} files  {data_bytes / 1048576:9.2f} MB   (separate artifact)")

    leaked = [r.as_posix() for r in core_files + data_files if r.name == "agent-config.json"]
    if leaked:
        print(f"refusing: machine-specific config would ship: {leaked}", file=sys.stderr)
        return 1
    if not args.write:
        print("\ndry run; add --write to build")
        return 0

    out_dir = (ROOT / args.out) if not Path(args.out).is_absolute() else Path(args.out)
    version_dir = out_dir / VERSION
    version_dir.mkdir(parents=True, exist_ok=True)

    core_zip = version_dir / f"StarCraftIIAgent-{VERSION}-core.zip"
    data_zip = version_dir / f"StarCraftIIAgent-{VERSION}-data.zip"

    # Order matters: the data pack must exist before its hash is embedded in core.zip.
    # Writing core first and hashing the data archive afterwards would only work when a
    # previous run happened to leave the file on disk.
    if not args.skip_data:
        write_zip(data_zip, data_files)

    skills = sorted(
        (
            {
                "name": r.parent.name,
                "source": r.parent.as_posix(),
                "group": r.parent.parent.name if len(r.parts) > 3 else None,
            }
            for r in core_files
            if r.name == "SKILL.md" and r.parts[0] == "skills"
        ),
        key=lambda s: s["name"],
    )
    inner = {
        "name": "StarCraftIIAgent",
        "version": VERSION,
        "layout_revision": LAYOUT_REVISION,
        "artifacts": {
            "data": {
                "file": None if args.skip_data else data_zip.name,
                "sha256": None if args.skip_data else sha256_file(data_zip),
                "uncompressed_bytes": data_bytes,
                "file_count": len(data_files),
                "required": False,
                "note": "Bulk DataEditorXML exports. Skipped when the installed copy already matches.",
            },
        },
        "files": {"core": {r.as_posix(): sha256_file(ROOT / r) for r in core_files},
                  "data": {r.as_posix(): sha256_file(ROOT / r) for r in data_files}},
        "skills": skills,
    }
    write_zip(core_zip, core_files, extra=inner)

    manifest = {
        "name": "StarCraftIIAgent",
        "version": VERSION,
        "layout_revision": LAYOUT_REVISION,
        "generated_utc": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "artifacts": {
            "core": {
                "file": core_zip.name,
                "bytes": core_zip.stat().st_size,
                "sha256": sha256_file(core_zip),
                "uncompressed_bytes": core_bytes,
                "file_count": len(core_files),
                "required": True,
            },
            "data": {
                "file": None if args.skip_data else data_zip.name,
                "bytes": 0 if args.skip_data else data_zip.stat().st_size,
                "sha256": None if args.skip_data else sha256_file(data_zip),
                "uncompressed_bytes": data_bytes,
                "file_count": len(data_files),
                "required": False,
                "note": "Bulk DataEditorXML exports. Skip when the installed copy already matches.",
            },
        },
        "files": {
            "core": {r.as_posix(): sha256_file(ROOT / r) for r in core_files},
            "data": {r.as_posix(): sha256_file(ROOT / r) for r in data_files},
        },
        # The harness convention is flat: <skills-dir>/<name>/SKILL.md.  The kit nests some
        # skills (skills/galaxy/galaxy-x/), so the installer needs the flat name and the
        # source directory to copy from.  parts[1] would say "galaxy" for every sub-skill.
        "skills": skills,
    }
    manifest_path = version_dir / f"StarCraftIIAgent-{VERSION}-manifest.json"
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=1) + "\n",
                             encoding="utf-8", newline="\n")

    print(f"\n  {core_zip.name}  {core_zip.stat().st_size / 1048576:.2f} MB")
    if not args.skip_data:
        print(f"  {data_zip.name}  {data_zip.stat().st_size / 1048576:.2f} MB")
    print(f"  {manifest_path.name}  {len(manifest['files']['core']) + len(manifest['files']['data'])} file hashes")
    print(f"  skills discovered: {len(manifest['skills'])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
