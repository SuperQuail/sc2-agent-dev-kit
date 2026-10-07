#!/usr/bin/env python3
r"""Temp-path normalisation shared by the pytest and unittest runners.

GitHub's Windows runners point TMP at an 8.3 short name:

    TMP = C:\Users\RUNNER~1\AppData\Local\Temp

while Path.resolve() expands it to the long form C:\Users\runneradmin\...  The
code under test resolves paths; the tests build expectations straight from
tempfile; the two then disagree as strings although they name the same directory.
Every path assertion fails for a reason no real user can hit.

This has to live outside conftest.py because the suite runs two ways:

  * pytest            - loads conftest.py, which calls normalise_temp_root()
  * unittest discover  - a subprocess with no conftest, so test-suite.py passes it
                         normalised_environ() instead

Normalising the temp root removes the whole class of failures on any machine whose
profile name is longer than eight characters.  It does not mask a bug: it makes the
test environment behave like an ordinary machine with no 8.3 alias.
"""
from __future__ import annotations

import os
import tempfile
from pathlib import Path

TEMP_VARS = ("TMP", "TEMP", "TMPDIR")


def long_temp_root() -> str:
    """The temp directory in its canonical (long) form."""
    return str(Path(tempfile.gettempdir()).resolve(strict=False))


def normalise_temp_root() -> str:
    """Rewrite this process's temp root to the canonical form. Returns it."""
    root = long_temp_root()
    tempfile.tempdir = root
    for name in TEMP_VARS:
        if name in os.environ:
            os.environ[name] = root
    return root


def normalised_environ() -> dict:
    """A copy of the environment with the temp vars canonical, for a child process."""
    env = dict(os.environ)
    root = long_temp_root()
    for name in TEMP_VARS:
        if name in env:
            env[name] = root
    return env


if __name__ == "__main__":
    print(normalise_temp_root())
