#!/usr/bin/env python3
"""Small helpers that more than one tool needs.

Hashing is all that lives here so far.  Four tools had each grown their own copy
of "sha256 this file", which is how a kit ends up with several subtly different
notions of what a file digest is: one reading the file whole, one hashing bytes
rather than a path, one chunked.  A release manifest has to hash files large
enough that reading them whole is wasteful, so the chunked form is the one that
ships here.
"""
from __future__ import annotations

import hashlib
from pathlib import Path

CHUNK = 1 << 20


def digest_bytes(payload: bytes) -> str:
    """SHA-256 of a byte string."""
    return hashlib.sha256(payload).hexdigest()


def digest_file(path: Path) -> str:
    """SHA-256 of a file, read in chunks so its size does not matter."""
    h = hashlib.sha256()
    with Path(path).open("rb") as fh:
        for chunk in iter(lambda: fh.read(CHUNK), b""):
            h.update(chunk)
    return h.hexdigest()
