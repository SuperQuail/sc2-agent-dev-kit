#!/usr/bin/env python3
"""Beginner-facing front end for the updater, packaged as a single .exe later.

Why this exists separately from update.py: update.py is a developer tool with
flags.  Someone who was handed a kit and a new release should not have to know a
flag from a path - they double-click this, it says what it found, and it asks one
question.  All the safety lives in update.py; this only chooses the target and
talks to a person.

Drag a release zip onto the .exe, or run it and type the path when asked.
"""
from __future__ import annotations

import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import sc2_console  # noqa: E402
import update  # noqa: E402


def frozen() -> bool:
    return bool(getattr(sys, "frozen", False))


def look_like_kit(path: Path) -> bool:
    return (path / "tools" / "sc2.py").is_file()


def resolve_kit_root(explicit: str | None) -> Path | None:
    """Find the installed kit, for both a script run and a frozen .exe.

    A one-file exe unpacks itself to a temp directory, so update.ROOT - which is
    derived from __file__ - points into that temp directory and not at the kit.
    The exe therefore looks next to itself, then one level up, which covers both
    "put the exe in the kit" and "put it beside the kit folder".
    """
    if explicit:
        candidate = Path(explicit).expanduser().resolve()
        return candidate if look_like_kit(candidate) else None
    if frozen():
        beside = Path(sys.executable).resolve().parent
        for candidate in (beside, beside.parent):
            if look_like_kit(candidate):
                return candidate
        return None
    return update.ROOT if look_like_kit(update.ROOT) else None


def ask(question: str, default: str = "") -> str:
    suffix = f" [{default}]" if default else ""
    try:
        answer = input(f"{question}{suffix}: ").strip()
    except (EOFError, KeyboardInterrupt):
        print()
        return ""
    return answer or default


def pause(message: str = "按回车键关闭窗口...") -> None:
    """Keep the window open when the exe was started by a double-click."""
    if not sys.stdin or not sys.stdin.isatty():
        return
    try:
        input(message)
    except (EOFError, KeyboardInterrupt):
        pass


def choose_source(argv: list[str]) -> str | None:
    if len(argv) > 1 and argv[1].strip():
        return argv[1].strip()
    return ask("把发行包 zip 拖到这里，或直接输入路径")


def run(argv: list[str]) -> int:
    sc2_console.enable_utf8_output()
    print("=" * 58)
    print("  StarCraftIIAgent 更新程序")
    print("=" * 58)

    root = resolve_kit_root(os.environ.get("SC2_KIT_ROOT"))
    if root is None:
        print("\n没有找到已安装的 kit（需要在包含 tools/sc2.py 的目录里运行）。")
        typed = ask("请输入 kit 目录的完整路径")
        root = resolve_kit_root(typed)
    if root is None:
        print("仍然找不到 tools/sc2.py，已退出。")
        return 2

    print(f"\n安装目录: {root}")
    print(f"当前版本: {update.installed_version(root)}")

    source = choose_source(argv)
    if not source:
        print("没有提供发行包，已退出。")
        return 2

    import tempfile
    try:
        with tempfile.TemporaryDirectory(prefix="sc2-updater-") as tmp, \
                update.load_release(source, Path(tmp)) as release:
            print(f"\n发行包  : {release.label}")
            print(f"发行版本: {release.version}")

            if release.layout_revision > update.sc2_version.LAYOUT_REVISION:
                print("\n这个发行包的目录结构变化太大，不能就地更新。")
                print("请解压完整发行包，覆盖安装目录。")
                return 1

            plan = update.build_plan(release, root)
            if not plan["added"] and not plan["changed"]:
                print("\n已经是最新版本，不需要更新。")
                return 0

            update.report_plan(release, plan, root, as_json=False)
            print()
            if ask("确认更新吗？输入 y 继续，其它键取消", "n").lower() not in ("y", "yes"):
                print("已取消，什么都没改。")
                return 0

            update.verify_payloads(release, plan)
            backup = update.apply_plan(release, plan, root)
    except update.UpdateError as exc:
        print(f"\n更新中止：{exc}")
        print("安装目录没有被改动，或已自动回滚。")
        return 1

    print(f"\n完成：已更新到 {release.version}")
    print(f"备份保留在 {backup}")
    print("出问题时，把备份目录里的文件按相同路径复制回去即可。")
    print("\n建议接着运行： python tools/test-suite.py --scope tools")
    return 0


def main() -> int:
    try:
        return run(sys.argv)
    finally:
        pause()


if __name__ == "__main__":
    raise SystemExit(main())
