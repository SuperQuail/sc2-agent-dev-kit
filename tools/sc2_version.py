#!/usr/bin/env python3
"""Single source of truth for the kit's version.

Why an importable module instead of a VERSION text file: three consumers need
this number and they drift the moment it lives in more than one place —

  * every tool's diagnostic output and the "sc2.py --version" flag
  * the packager, which stamps the archive name and the release manifest
  * the updater, which compares an installed version against a published one

Keeping it importable means the packager and the updater cannot disagree with
the number a user sees.  Bump VERSION here and nowhere else.
"""
from __future__ import annotations

import re

VERSION = "0.1.0a1"

# Bumped only when the shipped layout changes in a way an older updater cannot
# apply in place (a shipped directory renamed or moved).  The updater refuses an
# in-place apply across a higher value and asks for a full release instead of
# silently producing a half-updated tree.
LAYOUT_REVISION = 1


def version_tuple(text: str = VERSION) -> tuple[int, ...]:
    """Comparable form of a dotted version, ignoring any pre-release suffix.

    A suffix attaches directly to the last numeric component ("0.1.0a1"), so each
    component is parsed for its leading digits.  Splitting on "." and testing
    isdigit() would read "0a1" as non-numeric and collapse it to zero, which makes
    0.1.1a1 compare equal to 0.1.0 — an updater would then refuse a real update.
    """
    core = text.strip().split("-", 1)[0].split("+", 1)[0]
    out = []
    for chunk in core.split("."):
        digits = re.match(r"\d+", chunk)
        out.append(int(digits.group()) if digits else 0)
    return tuple(out)


def is_newer(candidate: str, current: str = VERSION) -> bool:
    """True when the candidate release is later than the current one."""
    return version_tuple(candidate) > version_tuple(current)


if __name__ == "__main__":
    print(VERSION)
