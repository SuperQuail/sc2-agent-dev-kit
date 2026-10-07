#!/usr/bin/env python3
"""Unified entry point for the StarCraft II modding workspace.

Why this exists: the workspace used to explain its rules in prose that every
agent had to read on every task.  Each verb below replaces a chunk of that prose
with a command.  "sc2.py rules" prints the static rules as data, so an agent
reads a short list instead of a manual, and "sc2.py check" enforces them so it
cannot apply one wrongly.

    python tools/sc2.py                  # list the verbs
    python tools/sc2.py rules            # the static rules, as data
    python tools/sc2.py check            # enforce them against the active project
    python tools/sc2.py init "<mod>"     # select/initialise a project
    python tools/sc2.py where            # which files the current mode lets you edit

Delegating verbs forward to the existing single-purpose tools so there is one
front door and no duplicated logic.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
sys.path.insert(0, str(TOOLS))

import sc2_checks  # noqa: E402
import sc2_console  # noqa: E402
import sc2_records  # noqa: E402
import sc2_version  # noqa: E402

CONFIG = ROOT / "agent-config.json"


def _delegate(script, argv):
    """Run an existing tool with the remaining arguments."""
    cmd = [sys.executable, str(TOOLS / script)] + list(argv)
    return subprocess.call(cmd, cwd=str(ROOT))


def cmd_rules(args):
    scopes = {}
    for entry in sc2_checks.RULES:
        scopes.setdefault(entry["scope"], []).append(entry)
    print("静态规则（唯一真源；检查器在 tools/sc2_checks.py）\n")
    for scope in ("xml", "galaxy", "any"):
        entries = scopes.get(scope)
        if not entries:
            continue
        print(f"[{scope}]")
        for e in entries:
            print(f"  {e['id']:<24} {e['rule']}")
            print(f"  {' ' * 24} 为什么: {e['why']}")
        print()
    print("跑检查：python tools/sc2.py check [--scope xml|galaxy|any|all] [target ...]")
    print("没有列在这里的约束＝还没做成工具。要么补检查器，要么查对应技能的 references/。")
    return 0


def cmd_check(args):
    roots = [Path(p) for p in args.targets] if args.targets else _default_roots()
    if not roots:
        print("没有可检查的目标：先运行 sc2.py init <主 .SC2Mod 路径>，或显式传路径。")
        return 2
    results, failures = sc2_checks.run(args.scope, roots)
    for entry, found in results:
        mark = "PASS" if not found else f"FAIL({len(found)})"
        print(f"  {mark:<10} {entry['id']:<24} {entry['rule']}")
        for line in found[:20]:
            print(f"             {line}")
        if len(found) > 20:
            print(f"             ... 还有 {len(found) - 20} 条")
    print(f"\n检查 {len(results)} 条规则，失败 {len(failures)} 处。")
    if failures:
        print("静态检查未通过——不要报告 static validation passed。", file=sys.stderr)
        return 1
    print("静态检查全绿。这只是 static validation passed，不等于 Editor accepted。")
    return 0


def _default_roots():
    """Sources to check when no explicit target is given."""
    if not CONFIG.is_file():
        return []
    try:
        cfg = json.loads(CONFIG.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return []
    project = cfg.get("project", {})
    mode = project.get("source_mode")
    key = "source_mod" if mode == "workspace_copy" else "primary_mod"
    raw = project.get(key)
    if not raw:
        return []
    path = Path(raw)
    if not path.is_absolute():
        path = (ROOT / path).resolve()
    return [path] if path.exists() else []


def cmd_start(args):
    """Hand over the previous task records. Run this before any development work."""
    directory = sc2_records.project_dir()
    found = sc2_records.sessions()
    print(f"项目    : {sc2_records.project_key()}")
    print(f"记录库  : {directory}")
    print("（记录库在工作区之外，不随分发包走，也不进版本库）\n")
    if not found:
        print("没有历史任务资料——这是该项目的第一次任务。")
    else:
        shown = found[-args.limit:][::-1]
        print(f"开工前先读这些历史任务资料（最近 {len(shown)} / 共 {len(found)} 条）:")
        for path in shown:
            print(f"  {sc2_records.first_heading(path)}")
            print(f"    {path}")
    for name, purpose in (
        ("findings.md", "已确认的持久发现"),
        ("decisions.md", "决策与理由"),
        ("issues.md", "问题账本（状态机）"),
    ):
        path = directory / name
        if path.is_file():
            lines = [l for l in path.read_text(encoding="utf-8", errors="replace").splitlines()
                     if l.strip() and not l.startswith("#")]
            if lines:
                print(f"\n{purpose}: {path}  （{len(lines)} 行）")
    print("\n读完再动手。新增记录用：python tools/sc2.py record new \"<标题>\"")
    return 0


def cmd_record(args):
    action = args.action
    if action == "path":
        print(sc2_records.project_dir())
        return 0
    if action == "list":
        found = sc2_records.sessions()
        if not found:
            print("还没有任何记录。用 record new \"<标题>\" 开始一条。")
            return 0
        for path in found[-args.limit:][::-1]:
            stamp = path.stem[:15]
            print(f"  {stamp}  {sc2_records.first_heading(path)}")
            print(f"                 {path}")
        return 0
    if action == "new":
        topic = " ".join(args.rest).strip()
        if not topic:
            print("用法: sc2.py record new \"<标题>\"", file=sys.stderr)
            return 2
        path = sc2_records.new_session(topic)
        print(path)
        return 0
    if action == "show":
        found = sc2_records.sessions()
        if not found:
            print("还没有任何记录。", file=sys.stderr)
            return 2
        target = found[-1]
        if args.rest:
            needle = args.rest[0]
            hit = [p for p in found if needle in p.name]
            if not hit:
                print(f"没有匹配 {needle!r} 的记录", file=sys.stderr)
                return 2
            target = hit[-1]
        print(target.read_text(encoding="utf-8", errors="replace"))
        return 0
    if action == "note":
        text = " ".join(args.rest).strip()
        if not text:
            print("用法: sc2.py record note \"<内容>\"", file=sys.stderr)
            return 2
        print(sc2_records.append_session(text))
        return 0
    print(f"未知 record 动作: {action}", file=sys.stderr)
    return 2


def cmd_find_id(args):
    """Does this catalog ID already exist? Run before inventing a new one.

    Replaces the prose rule "check the dependency chain for an existing
    validator/entity first" with a search that also looks in the reference
    exports, which is where an official definition usually already lives.
    """
    needle = args.identifier
    pattern = re.compile(r'id\s*=\s*"' + re.escape(needle) + r'"')
    roots = [Path(p) for p in args.targets] if args.targets else _default_roots()
    roots.append(ROOT / "DataEditorXML")
    hits = []
    seen = set()
    for root in roots:
        if not root.exists():
            continue
        for path in (root.rglob("*.xml") if root.is_dir() else [root]):
            if not path.is_file() or SKIP.search(str(path)):
                continue
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            if needle in text and pattern.search(text):
                key = path.name
                if key in seen:
                    continue
                seen.add(key)
                hits.append(path.relative_to(ROOT).as_posix())
    if not hits:
        print(f"{needle}: 没有找到既有定义——可以安全新建。")
        return 0
    print(f"{needle}: 已存在 {len(hits)} 处定义：")
    for h in hits[:args.limit]:
        print(f"  {h}")
    if len(hits) > args.limit:
        print(f"  ... 还有 {len(hits) - args.limit} 处（--limit 调整）")
    print("\n先看既有定义再决定新建还是继承，避免重复。")
    return 0


SKIP = re.compile(r"[\\/](DataEditorXML[\\/]SC2GameDataComponents)?[\\/]?\.git[\\/]|__pycache__")

def cmd_where(args):
    if not CONFIG.is_file():
        print("还没有 agent-config.json：先运行 sc2.py init <主 .SC2Mod 路径>")
        return 2
    cfg = json.loads(CONFIG.read_text(encoding="utf-8"))
    project = cfg.get("project", {})
    mode = project.get("source_mode", "(未设置)")
    print(f"source_mode : {mode}")
    if mode == "in_place":
        print("改哪里      : 主模组本体")
        print(f"主模组      : {project.get('primary_mod', '(未设置)')}")
    elif mode == "workspace_copy":
        print("改哪里      : 工作区副本（改完用 sc2.py deploy 部署）")
        print(f"副本        : {project.get('source_mod', '(未设置)')}")
        if not project.get("source_mod"):
            print("副本未设置——按规则应停止写入。")
            return 1
    print("永不编辑    : publish/ 、Lib*.galaxy 、MapScript.galaxy")
    return 0


VERBS = [
    ("start",   "开工前先读历史任务资料",            cmd_start),
    ("rules",   "列出全部静态规则及其检查器",        cmd_rules),
    ("check",   "强制静态规则",                      cmd_check),
    ("where",   "打印按 source_mode 该改哪里",        cmd_where),
    ("record",  "本地开发记录（list/new/show/note/path）", cmd_record),
    ("find-id", "查某个 catalog ID 是否已存在",         cmd_find_id),
]

DELEGATES = {
    "init":      ("init-project.py",              "初始化/切换项目"),
    "tests":     ("test-suite.py",                "预飞行测试套件"),
    "validate":  ("validate-mod.py",              "XML/Galaxy 静态校验"),
    "query":     ("sc2-catalog-query.py",         "查询 catalog 图"),
    "reference": ("sc2-reference-query.py",       "查官方/合作组件样例"),
    "issues":    ("audit-issue-lifecycle.py",     "校验问题账本状态"),
    "strings":   ("audit-gamestrings-anchors.py", "字符串锚点审计"),
    "cards":     ("audit-actor-and-card-integrity.py", "命令卡完整性审计"),
    "graph":     ("build-sc2-catalog-graph.py",   "重建 catalog 图数据库"),
    "dumps":     ("build-data-editor-dumps.py",   "派生 DataEditorXML 顶层导出"),
    "deploy":    ("deploy-mod.py",                "部署模组到 SC2"),
    "bugreport": ("extract-playtest-bugreport.py","提取游戏测试错误"),
    "links":     ("check-doc-links.py",           "检查文档链接"),
    "package":   ("package.py",                   "打包可分发给其他用户的内容"),
}


def main():
    sc2_console.enable_utf8_output()
    argv = sys.argv[1:]
    if not argv or argv[0] in ("-h", "--help", "help"):
        print(__doc__.strip().splitlines()[0])
        print("\n用法: python tools/sc2.py <动作> [参数...]\n")
        print("内建动作:")
        for name, desc, _ in VERBS:
            print(f"  {name:<10} {desc}")
        print(f"\n版本: StarCraftIIAgent {sc2_version.VERSION}")
        print("\n转发动作:")
        for name, (script, desc) in DELEGATES.items():
            print(f"  {name:<10} {desc}")
        print("\n先跑 `sc2.py rules` 看规则，再跑 `sc2.py check` 验证。")
        return 0 if argv else 0

    verb, rest = argv[0], argv[1:]
    if verb in ("--version", "-V", "version"):
        print(f"StarCraftIIAgent {sc2_version.VERSION}")
        return 0
    if verb == "rules":
        return cmd_rules(None)
    if verb == "check":
        p = argparse.ArgumentParser(prog="sc2.py check")
        p.add_argument("--scope", default="all", choices=("all", "xml", "galaxy", "any"))
        p.add_argument("targets", nargs="*")
        return cmd_check(p.parse_args(rest))
    if verb == "where":
        return cmd_where(None)
    if verb == "find-id":
        p = argparse.ArgumentParser(prog="sc2.py find-id")
        p.add_argument("identifier")
        p.add_argument("--limit", type=int, default=12)
        p.add_argument("targets", nargs="*")
        return cmd_find_id(p.parse_args(rest))
    if verb == "start":
        p = argparse.ArgumentParser(prog="sc2.py start")
        p.add_argument("--limit", type=int, default=8)
        return cmd_start(p.parse_args(rest))
    if verb == "record":
        p = argparse.ArgumentParser(prog="sc2.py record")
        p.add_argument("action", choices=("list", "new", "show", "note", "path"))
        p.add_argument("--limit", type=int, default=15)
        p.add_argument("rest", nargs="*")
        return cmd_record(p.parse_args(rest))
    if verb in DELEGATES:
        script = DELEGATES[verb][0]
        if not (TOOLS / script).is_file():
            print(f"缺少 tools/{script}（该动作尚未就位）", file=sys.stderr)
            return 2
        return _delegate(script, rest)

    print(f"未知动作: {verb}\n跑 `python tools/sc2.py` 看可用动作。", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
