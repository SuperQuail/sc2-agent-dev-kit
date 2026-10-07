#!/usr/bin/env python3
"""Derive DataEditorXML/*.txt dumps from the SC2GameDataComponents snapshot.

The 156 top-level dumps are a convenience grep surface: human-readable names at
a shallow path, so an agent can be told "grep Core Abilities.txt".  They are not
independent data.  154 of them reproduce exactly from the snapshot -- 61
verbatim, 93 with the trailing newline dropped -- which makes the pair a 9.3 MB
duplicate and a drift risk.  This script is the derivation, so the dumps can be
generated on demand instead of committed.

    python tools/build-data-editor-dumps.py            # check (default)
    python tools/build-data-editor-dumps.py --write    # regenerate

The two dumps under "manual" in the manifest match no snapshot file and are
never written here.

Exit codes: 0 clean, 1 drift or unknown dump, 2 bad invocation or manifest.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

MANIFEST_REL = "tools/data-editor-dumps.json"
DUMP_DIR = "DataEditorXML"


def apply_transform(blob: bytes, kind: str) -> bytes:
    """Reproduce a dump from its snapshot source."""
    if kind == "verbatim":
        return blob
    if kind == "strip-trailing-newline":
        if blob.endswith(b"\r\n"):
            return blob[:-2]
        if blob.endswith(b"\n"):
            return blob[:-1]
        return blob
    raise ValueError(f"unknown transform {kind!r} in the manifest")


def load_manifest(root: Path) -> dict:
    path = root / MANIFEST_REL
    if not path.is_file():
        raise SystemExit(f"error: manifest not found: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--root", default=None, help="repository root (default: this script's parent)")
    parser.add_argument("--write", action="store_true", help="write the dumps instead of checking")
    parser.add_argument("--only", default=None, help="only touch dumps whose name contains this text")
    parser.add_argument("--quiet", action="store_true", help="print the summary only")
    args = parser.parse_args()

    root = Path(args.root).resolve() if args.root else Path(__file__).resolve().parents[1]
    manifest = load_manifest(root)
    snapshot_root = root / manifest["snapshot_root"]
    dump_dir = root / DUMP_DIR
    dumps: dict[str, dict] = manifest["dumps"]
    manual: dict[str, str] = manifest.get("manual", {})

    if not snapshot_root.is_dir():
        print(f"error: snapshot missing: {snapshot_root}", file=sys.stderr)
        print("       DataEditorXML dumps cannot be derived without it.", file=sys.stderr)
        return 2

    selected = {n: s for n, s in dumps.items() if not args.only or args.only in n}
    ok = 0
    drift: list[str] = []
    missing_source: list[str] = []
    rewritten: list[str] = []

    for name, spec in sorted(selected.items()):
        source = snapshot_root / spec["source"]
        if not source.is_file():
            missing_source.append(f"{name}: source not found: {spec['source']}")
            continue
        expected = apply_transform(source.read_bytes(), spec["transform"])
        target = dump_dir / name
        current = target.read_bytes() if target.is_file() else None
        if current == expected:
            ok += 1
            continue
        if args.write:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(expected)
            rewritten.append(name)
            continue
        if current is None:
            drift.append(f"{name}: missing; expected {len(expected)} bytes from {spec['source']}")
        else:
            drift.append(
                f"{name}: differs; expected {len(expected)} bytes from {spec['source']} "
                f"({spec['transform']}), on disk {len(current)} bytes"
            )

    known = set(dumps) | set(manual)
    unknown = sorted(
        p.name for p in dump_dir.glob("*.txt") if p.name not in known
    )

    if args.write:
        for name in rewritten:
            if not args.quiet:
                print(f"wrote {name}")
        print(f"derivable {len(selected)}  ok {ok}  written {len(rewritten)}  manual {len(manual)}")
    else:
        for line in drift:
            print(f"DRIFT {line}")
        for line in missing_source:
            print(f"SOURCE {line}")
        for name in unknown:
            print(f"UNKNOWN {name} (not in manifest: neither derivable nor listed under manual)")
        print(
            f"derivable {len(selected)}  ok {ok}  drift {len(drift)}  "
            f"missing-source {len(missing_source)}  manual {len(manual)}  unknown {len(unknown)}"
        )

    if drift or missing_source or unknown:
        if not args.write:
            print("run with --write to regenerate the derivable dumps", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
