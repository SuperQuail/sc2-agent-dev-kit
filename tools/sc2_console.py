#!/usr/bin/env python3
"""Make the kit's Chinese output readable on Windows.

Windows Python does not use the console code page for its own stdio.  With the
console switched to UTF-8 (chcp 65001) Python still encodes stdout as the ANSI
code page (cp936 on this machine), so every Chinese message reaches the terminal
as mojibake - and on a default console the mismatch runs the other way.  Both
halves have to agree, so this sets both:

  * the console output code page, when the process is attached to a console
  * the encoding of sys.stdout / sys.stderr

errors="replace" is deliberate: a tool must never die because one glyph could not
be encoded, and a single "?" in a log line is recoverable while a traceback is
not.  Import-safe and idempotent - calling it twice does nothing.
"""
from __future__ import annotations

import sys

_APPLIED = False


def enable_utf8_output() -> None:
    """Point the console and Python's stdio at UTF-8. Safe to call repeatedly."""
    global _APPLIED
    if _APPLIED:
        return
    _APPLIED = True

    for stream in (sys.stdout, sys.stderr):
        reconfigure = getattr(stream, "reconfigure", None)
        if reconfigure is None:
            continue
        try:
            reconfigure(encoding="utf-8", errors="replace")
        except (OSError, ValueError):
            pass

    if sys.platform != "win32":
        return
    try:
        import ctypes

        kernel32 = ctypes.windll.kernel32
        # Zero means no console is attached (output is redirected); the encoding
        # set above is then the only thing that matters.
        if kernel32.GetConsoleOutputCP():
            kernel32.SetConsoleOutputCP(65001)
            kernel32.SetConsoleCP(65001)
    except Exception:  # noqa: BLE001 - a cosmetic fix must never break a tool
        pass


if __name__ == "__main__":
    enable_utf8_output()
    print("控制台已切到 UTF-8。中文测试 OK")
