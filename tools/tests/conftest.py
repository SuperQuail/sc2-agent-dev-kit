"""Test-suite wide setup.

GitHub's Windows runners point TMP at an 8.3 short name (C:\\Users\\RUNNER~1\\...),
while Path.resolve() expands it to the long form (C:\\Users\\runneradmin\\...).  The
code under test resolves paths, the tests build expectations straight from tempfile,
and so the two disagree as strings even though they name the same directory.  Every
path assertion then fails for a reason no real user can hit.

Normalising the temp root once, before any test creates a directory, removes the
whole class of failures.  It does not mask a bug: it makes the test environment
behave like an ordinary machine, where the temp path has no 8.3 alias.  A user whose
profile name is longer than eight characters gets the long form anyway — resolve()
expands it — so the long form is the honest baseline.
"""
from __future__ import annotations

import os
import tempfile
from pathlib import Path

_long_root = str(Path(tempfile.gettempdir()).resolve(strict=False))
tempfile.tempdir = _long_root
for _name in ("TMP", "TEMP", "TMPDIR"):
    if _name in os.environ:
        os.environ[_name] = _long_root


def pytest_report_header(config):
    """Surface the normalisation in the run header, so a surprising result is traceable."""
    return f"temp root normalised to {_long_root}"
