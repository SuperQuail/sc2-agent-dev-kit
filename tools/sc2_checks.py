#!/usr/bin/env python3
"""Static rule checks for StarCraft II mod sources.

Every entry in RULES is a prose rule from the workspace documentation that has
been turned into an executable check.  The point is token cost: an agent that
runs a checker does not have to read, remember, and correctly apply a paragraph
of rules, and it cannot silently misremember one.

Each checker takes a list of source roots and returns a list of failure strings
(empty means pass).  RULES is the single source of truth for what the rules are;
run "python tools/sc2.py rules" to print it.
"""
from __future__ import annotations

import re
import xml.etree.ElementTree as ET
from pathlib import Path

# Reference exports, tool internals and build output are not our sources.  Without
# this, checking a workspace root also checks 2564 exported Blizzard components and
# reports their localisation and legacy patterns as our failures.
SKIP_PARTS = {".git", "DataEditorXML", "publish", "__pycache__", ".venv", "node_modules", "vendor"}

ASCII_RE = re.compile(r"[^\x00-\x7F]")
INCLUDE_RE = re.compile(r'include\s+"([^"]+)"')
GALAXY_GENERATED_RE = re.compile(r"^(Lib.*\.galaxy|MapScript\.galaxy)$", re.IGNORECASE)

BANNED_XML = [
    ("AbilAutoCmd", "AbilAutoCmd is not a valid button Type in the SC2 XML schema; do not add it"),
    ("<InitEffect", "CBehaviorBuff does not support InitEffect; do not design around it"),
]


def _skipped(path, root):
    """True when the path sits under an excluded directory relative to its root."""
    try:
        parts = path.relative_to(root).parts
    except ValueError:
        parts = path.parts
    return bool(SKIP_PARTS.intersection(parts))


def _walk(roots, suffix):
    for root in roots:
        root = Path(root)
        if root.is_file():
            if root.suffix.lower() == suffix and not _skipped(root, root.parent):
                yield root
        elif root.is_dir():
            for path in root.rglob(f"*{suffix}"):
                if not _skipped(path, root):
                    yield path


def _xml_files(roots):
    yield from _walk(roots, ".xml")


def _galaxy_files(roots):
    yield from _walk(roots, ".galaxy")


def check_xml_ascii(roots):
    """XML comments and attribute values must be ASCII-only."""
    out = []
    for path in _xml_files(roots):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError as exc:
            out.append(f"{path}: cannot read ({exc})"); continue
        for n, line in enumerate(text.splitlines(), 1):
            stripped = line.lstrip()
            is_comment = stripped.startswith("<!--")
            is_attr = re.search(r'\w+="[^"]*"', line) and not stripped.startswith("<?"),
            if (is_comment or is_attr) and ASCII_RE.search(line):
                out.append(f"{path}:{n}: non-ASCII in XML comment/attribute: {line.strip()[:70]!r}")
    return out


def check_xml_banned(roots):
    """Patterns SC2 rejects outright."""
    out = []
    for path in _xml_files(roots):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for n, line in enumerate(text.splitlines(), 1):
            for needle, why in BANNED_XML:
                if needle in line:
                    out.append(f"{path}:{n}: {why}")
    return out


def check_xml_catalog_not_empty(roots):
    """A catalog made only of comments is rejected by the SC2 parser."""
    out = []
    for path in _xml_files(roots):
        try:
            root = ET.parse(path).getroot()
        except ET.ParseError:
            continue          # a syntax error is reported by the schema validator
        except OSError as exc:
            out.append(f"{path}: cannot read ({exc})"); continue
        if root.tag != "Catalog":
            continue
        entries = [c for c in root if isinstance(c.tag, str)]
        if not entries:
            out.append(f"{path}: catalog has no entries — a comment-only catalog is rejected by SC2")
    return out


def check_trigger_gui_first(roots):
    """Triggers should use editor-editable GUI actions, not large Custom Script blocks.

    The documented rule is a judgement call, but the common failure is mechanical:
    a Custom Script action carrying a whole routine that the GUI could express.
    Flag blocks past the threshold so the reviewer looks, rather than trusting
    that the rule was remembered.
    """
    out = []
    for root in roots:
        base = Path(root)
        for path in (base.rglob("*.xml") if base.is_dir() else [base]):
            if _skipped(path, base if base.is_dir() else path.parent):
                continue
            if path.suffix.lower() != ".xml" or "trigger" not in path.name.lower():
                continue
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for m in re.finditer(r"<ScriptCode[^>]*>(.*?)</ScriptCode>", text, re.S | re.I):
                body = m.group(1)
                lines = [l for l in body.splitlines() if l.strip()]
                if len(lines) > 25:
                    line_no = text[: m.start()].count("\n") + 1
                    out.append(
                        f"{path}:{line_no}: Custom Script block is {len(lines)} lines — "
                        "prefer GUI actions; only a small part that the GUI cannot express belongs here"
                    )
    return out


def check_galaxy_include(roots):
    """Include directives must be extensionless relative paths."""
    out = []
    for path in _galaxy_files(roots):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for n, line in enumerate(text.splitlines(), 1):
            if line.lstrip().startswith("//"):
                continue
            for target in INCLUDE_RE.findall(line):
                if target.endswith(".galaxy"):
                    out.append(f"{path}:{n}: include must omit the extension: include \"{target[:-7]}\"")
                elif target.startswith("/") or ".." in Path(target).parts:
                    out.append(f"{path}:{n}: include must be a relative in-tree path: include \"{target}\"")
    return out


def check_protected_files(roots):
    """Generated Galaxy output must not be hand-edited (it is overwritten on save)."""
    out = []
    for path in _galaxy_files(roots):
        if GALAXY_GENERATED_RE.match(path.name):
            out.append(
                f"{path}: generated Galaxy output — edit the Triggers source or Base.SC2Data/Scripts instead"
            )
    return out


GALAXY_LINE_LIMIT = 2048

# A trigger handler is registered by its own name as a string, so the language
# cannot check the link for us.
TRIGGER_CREATE_RE = re.compile(r'TriggerCreate\s*\(\s*"([^"]+)"')
GALAXY_DEF_RE = re.compile(
    r"^[ \t]*(?:bool|void|int|fixed|string|text|unit|point|trigger)\s+(\w+)\s*\(([^)]*)\)", re.M
)


def check_galaxy_utf8_no_bom(roots):
    """A .galaxy file saved with a UTF-8 BOM fails to load, and the error misleads.

    The engine reports "Scri: Script load failed: Function not found" and names a
    function that visibly exists, which sends the reader hunting for a code bug.
    The cause is a property of the first three bytes, so it is checked instead of
    written down and hoped for.
    """
    out = []
    for path in _galaxy_files(roots):
        try:
            with path.open("rb") as fh:
                head = fh.read(3)
        except OSError as exc:
            out.append(f"{path}: cannot read ({exc})")
            continue
        if head == b"\xef\xbb\xbf":
            out.append(
                f'{path}: saved as UTF-8 with BOM — re-save as UTF-8 without BOM, '
                'or the engine reports "Function not found" for a function that exists'
            )
    return out


def check_galaxy_line_length(roots):
    """Galaxy's compiler rejects a source line past 2048 characters."""
    out = []
    for path in _galaxy_files(roots):
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        for n, line in enumerate(text.splitlines(), 1):
            if len(line) > GALAXY_LINE_LIMIT:
                out.append(
                    f"{path}:{n}: line is {len(line)} characters — "
                    f"the compiler's hard limit is {GALAXY_LINE_LIMIT}"
                )
    return out


def _handler_signature_ok(params):
    parts = [p.strip() for p in params.split(",") if p.strip()]
    return len(parts) == 2 and all(p.split()[0] == "bool" for p in parts if p.split())


def check_galaxy_trigger_create_name(roots):
    """TriggerCreate takes a function name as a string, so nothing else checks it.

    A rename in the editor or a typo here yields a trigger that never fires and no
    diagnostic at all.  A string argument is invisible to the compiler, which is
    exactly the kind of rule worth turning into a checker.
    """
    files = list(_galaxy_files(roots))
    definitions = {}
    texts = {}
    for path in files:
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        texts[path] = text
        for m in GALAXY_DEF_RE.finditer(text):
            definitions.setdefault(m.group(1), set()).add(m.group(2))
    if not definitions:
        return []

    out = []
    for path, text in texts.items():
        for n, line in enumerate(text.splitlines(), 1):
            if line.lstrip().startswith("//"):
                continue
            for name in TRIGGER_CREATE_RE.findall(line):
                signatures = definitions.get(name)
                if signatures is None:
                    out.append(
                        f'{path}:{n}: TriggerCreate("{name}") — no such function in the scanned '
                        "Galaxy sources; the trigger would never fire"
                    )
                elif not any(_handler_signature_ok(s) for s in signatures):
                    out.append(
                        f'{path}:{n}: TriggerCreate("{name}") resolves to ({sorted(signatures)[0]}) — '
                        f"a trigger handler must be bool {name}(bool, bool)"
                    )
    return out


RULES = [
    {
        "id": "xml.ascii",
        "scope": "xml",
        "rule": "XML 注释与属性值只用 ASCII 字符",
        "why": "SC2 解析器拒绝非 ASCII 的注释/属性（智能引号、破折号、箭头是常见来源）",
        "check": check_xml_ascii,
    },
    {
        "id": "xml.banned",
        "scope": "xml",
        "rule": "禁用构造：AbilAutoCmd 按钮类型 / CBehaviorBuff 的 InitEffect",
        "why": "前者不在 schema 中；后者 SC2 不支持",
        "check": check_xml_banned,
    },
    {
        "id": "xml.catalog-not-empty",
        "scope": "xml",
        "rule": "每个 catalog 必须有实际条目",
        "why": "纯注释的 catalog 会被解析器拒绝",
        "check": check_xml_catalog_not_empty,
    },
    {
        "id": "trigger.gui-first",
        "scope": "xml",
        "rule": "触发器优先用 GUI action，Custom Script 只放 GUI 表达不了的少量逻辑",
        "why": "整段逻辑塞进 Custom Script 会绕过触发器编辑器，后续无法用 GUI 维护；超过 25 行的块会被标出待复核",
        "check": check_trigger_gui_first,
    },
    {
        "id": "galaxy.include",
        "scope": "galaxy",
        "rule": "include 使用无扩展名的相对路径",
        "why": "带 .galaxy 后缀或跳出树外的路径会导致链接失败",
        "check": check_galaxy_include,
    },
    {
        "id": "galaxy.utf8-no-bom",
        "scope": "galaxy",
        "rule": "Galaxy 源文件存为不带 BOM 的 UTF-8",
        "why": '带 BOM 的文件在加载时报 "Function not found"，指向一个明明存在的函数，把人引向代码 bug',
        "check": check_galaxy_utf8_no_bom,
    },
    {
        "id": "galaxy.line-length",
        "scope": "galaxy",
        "rule": "Galaxy 单行不超过 2048 个字符",
        "why": "编译器硬限制，超长行直接编译失败",
        "check": check_galaxy_line_length,
    },
    {
        "id": "galaxy.trigger-name",
        "scope": "galaxy",
        "rule": 'TriggerCreate("Name") 必须指向同编译单元里的 bool Name(bool, bool)',
        "why": "触发器按字符串名注册，编译器查不到；改名或打错只会让触发器永不触发且无任何报错",
        "check": check_galaxy_trigger_create_name,
    },
    {
        "id": "guard.protected",
        "scope": "any",
        "rule": "不手改 Lib*.galaxy / MapScript.galaxy",
        "why": "编译产物，编辑器保存时会被覆盖",
        "check": check_protected_files,
    },
]


def run(scope, roots):
    """Run every rule in scope. Returns (results, failures)."""
    results = []
    failures = []
    for entry in RULES:
        if scope not in ("all", "any", entry["scope"]):
            continue
        found = entry["check"](roots)
        results.append((entry, found))
        failures.extend(found)
    return results, failures
