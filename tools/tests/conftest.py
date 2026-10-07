"""pytest entry point for the temp-path normalisation.

The rationale, and the tests it protects, live in tools/sc2_temp.py.  Keeping the
logic there means the unittest subprocess started by tools/test-suite.py applies the
same rule - conftest.py is a pytest-only mechanism and unittest never reads it,
which is exactly how this reached CI as pytest-green / test-suite-red.
"""
from __future__ import annotations

import sys
from pathlib import Path

TOOLS_DIR = Path(__file__).resolve().parents[1]
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

from sc2_temp import normalise_temp_root  # noqa: E402

ROOT = normalise_temp_root()


def pytest_report_header(config):
    """Surface the normalisation, so a surprising result stays traceable."""
    return f"temp root normalised to {ROOT}"
