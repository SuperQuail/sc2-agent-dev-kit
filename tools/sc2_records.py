#!/usr/bin/env python3
"""Local development records, stored outside the kit.

Why not in the workspace: the workspace is a distributed kit.  If the agent
writes its issue ledger, status and session notes into it, then every task
dirties a shipped directory, the records vanish when the kit is updated, and
records end up interleaved with knowledge that is supposed to be read-only.

So records live beside the kit, inside the local workspace, keyed by project:

    <workspace>/sc2agent-records/<project-key>/
        sessions/<stamp>-<slug>.md    one file per task
        issues.md                     the issue ledger, state machine enforced
        findings.md                   durable facts worth keeping
        decisions.md                  decisions and why

<workspace> is the directory holding the kit, so records stay on the same drive
as the work instead of landing in a user profile on C:.  SC2AGENT_HOME overrides
the location.

Skills point here first: read the previous task records before starting work.
"""
from __future__ import annotations

import json
import os
import re
import sys
from datetime import datetime
from pathlib import Path

WORKSPACE = Path(__file__).resolve().parents[1]
CONFIG = WORKSPACE / "agent-config.json"

STAGES = [
    "reported",
    "root cause confirmed",
    "source fixed",
    "static validation passed",
    "Editor accepted",
    "packaged runtime passed",
]


RECORDS_DIRNAME = "sc2agent-records"


def home() -> Path:
    """Where records live: the local workspace, beside the kit.

    Deliberately not a user-profile directory.  The work sits on whatever drive
    the project lives on, and development records belong next to it rather than
    in AppData on C:.  Being a sibling of the kit also keeps them out of the
    distributed package while staying inside the workspace the agent may write.
    """
    override = os.environ.get("SC2AGENT_HOME")
    if override:
        return Path(override).expanduser()
    return WORKSPACE.parent / RECORDS_DIRNAME


def project_key() -> str:
    """Stable, filesystem-safe key for the active project."""
    raw = None
    if CONFIG.is_file():
        try:
            cfg = json.loads(CONFIG.read_text(encoding="utf-8"))
            project = cfg.get("project", {})
            raw = project.get("primary_mod") or project.get("source_mod")
        except (OSError, ValueError):
            raw = None
    if not raw:
        return "unassigned"
    name = Path(raw).name or "project"
    safe = re.sub(r"[^A-Za-z0-9._-]+", "-", name).strip("-") or "project"
    return safe


ISSUE_TEMPLATE = """# 问题账本

一条问题一个下列块。状态只能沿 STAGES 前进，禁止越级。

## ISSUE-001: <标题>

- **Status:** reported
- **Map / Context:**
- **Reproduction:**
- **Observed:**
- **Expected:**
- **Root Cause:**
- **Fix:**
- **Validation:**
"""


def project_dir(create=True) -> Path:
    path = home() / project_key()
    if create:
        (path / "sessions").mkdir(parents=True, exist_ok=True)
        for name in ("issues.md", "findings.md", "decisions.md"):
            f = path / name
            if not f.exists():
                body = ISSUE_TEMPLATE if name == "issues.md" else f"# {name[:-3]}\n"
                f.write_text(body, encoding="utf-8", newline="\n")
    return path


def slug(text: str, limit=48) -> str:
    """Filesystem-safe slug that keeps CJK.

    The obvious [^A-Za-z0-9._-] filter erases a Chinese title completely and
    collapses every session to the same name, which is exactly the audience this
    kit targets.  Word characters are kept, which includes CJK under re.UNICODE.
    """
    safe = re.sub(r"[^\w.-]+", "-", text.strip(), flags=re.UNICODE).strip("-._")
    return (safe[:limit] or "session")


def new_session(title: str) -> Path:
    stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    path = project_dir() / "sessions" / f"{stamp}-{slug(title)}.md"
    body = "\n".join([
        f"# {title}",
        "",
        f"- 开始: {datetime.now().strftime('%Y-%m-%d %H:%M')}",
        f"- 项目: {project_key()}",
        "- 状态: in_progress",
        "",
        "## 目标",
        "",
        "",
        "## 记录",
        "",
        "",
        "## 结论",
        "",
        "",
    ])
    path.write_text(body, encoding="utf-8", newline="\n")
    return path


def sessions() -> list[Path]:
    d = project_dir() / "sessions"
    return sorted(d.glob("*.md")) if d.is_dir() else []


def latest_session() -> Path | None:
    found = sessions()
    return found[-1] if found else None


def append_session(text: str) -> Path:
    path = latest_session()
    if path is None:
        path = new_session("session")
    stamp = datetime.now().strftime("%H:%M")
    with path.open("a", encoding="utf-8", newline="\n") as fh:
        fh.write(f"- {stamp} {text}\n")
    return path


def first_heading(path: Path) -> str:
    try:
        for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
            if line.startswith("# "):
                return line[2:].strip()
    except OSError:
        pass
    return path.stem

