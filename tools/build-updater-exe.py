#!/usr/bin/env python3
"""Build the beginner-facing updater as a single .exe.

Why ship an .exe at all: the people this kit is handed to are not expected to have
a working python on PATH, and "open a terminal and run python tools/update.py"
is where a non-developer stops.  The exe bundles the interpreter, so updating is
double-click, drop in a zip, answer one question.

PyInstaller is a build-time dependency, not a runtime one - the built exe needs
nothing installed.

    python tools/build-updater-exe.py            # dist/updater/SC2Agent-Updater.exe
    python tools/build-updater-exe.py --onedir   # folder build: starts faster,
                                                 # but is not a single file to hand over
    python tools/build-updater-exe.py --no-smoke # skip running the built exe

Note for whoever distributes this: a freshly built one-file exe is sometimes held
back by antivirus for a few minutes because it unpacks itself at start-up.  That
is a false positive; a folder build (--onedir) avoids it entirely.
"""
from __future__ import annotations

import argparse
import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import sc2_console  # noqa: E402
import sc2_version  # noqa: E402

TOOLS = Path(__file__).resolve().parent
ROOT = TOOLS.parent
ENTRY = TOOLS / "updater_exe.py"
APP_NAME = "SC2Agent-Updater"


def pyinstaller_available() -> bool:
    try:
        done = subprocess.run([sys.executable, "-m", "PyInstaller", "--version"],
                              capture_output=True, text=True, timeout=120)
    except (OSError, subprocess.SubprocessError):
        return False
    return done.returncode == 0


def build(onefile: bool, clean: bool) -> Path:
    out = ROOT / "dist" / "updater"
    work = ROOT / "build" / "updater"
    work.mkdir(parents=True, exist_ok=True)
    cmd = [
        sys.executable, "-m", "PyInstaller",
        "--name", APP_NAME,
        "--console",
        "--noconfirm",
        "--distpath", str(out),
        "--workpath", str(work),
        "--specpath", str(work),
        # updater_exe.py does sys.path.insert on its own directory; telling the
        # analyser where that is keeps the sibling modules discoverable.
        "--paths", str(TOOLS),
        "--onefile" if onefile else "--onedir",
    ]
    if clean:
        cmd.append("--clean")
    cmd.append(str(ENTRY))
    print("运行: " + " ".join(cmd))
    done = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True,
                          encoding="utf-8", errors="replace")
    if done.returncode != 0:
        tail = "\n".join((done.stderr or done.stdout or "").strip().splitlines()[-15:])
        raise RuntimeError("PyInstaller 构建失败：\n" + tail)
    suffix = ".exe" if os.name == "nt" else ""
    exe = out / (APP_NAME + suffix) if onefile else out / APP_NAME / (APP_NAME + suffix)
    if not exe.is_file():
        raise RuntimeError(f"构建结束但找不到产物：{exe}")
    return exe


def make_release_fixture(workdir: Path, version: str) -> Path:
    """Build a throwaway release so the exe can be exercised for real."""
    import package
    files = package.collect(include_data=False)
    extra = package.synthetic_payloads(include_data=False)
    manifest = package.build_manifest(files, version, include_data=False, extra=extra)
    archive = workdir / f"StarCraftIIAgent-{version}.zip"
    package.write_archive(archive, files, f"StarCraftIIAgent-{version}", version, manifest, extra)
    return archive


def smoke_test(exe: Path) -> list[str]:
    """Run the built exe against a real kit and a real release.

    A build that produces a file is not a build that works: the frozen entry point
    resolves the kit from sys.executable rather than __file__, and only running it
    proves that path logic survived freezing.
    """
    import zipfile
    problems = []
    with tempfile.TemporaryDirectory(prefix="sc2-exe-smoke-") as tmp:
        work = Path(tmp)
        release = make_release_fixture(work, sc2_version.VERSION)
        kit = work / "kit"
        with zipfile.ZipFile(release) as zf:
            zf.extractall(work / "unpack")
        unpacked = [p for p in (work / "unpack").iterdir() if p.is_dir()]
        if len(unpacked) != 1:
            return ["发行包解压结构异常，无法做冒烟测试"]
        shutil.move(str(unpacked[0]), str(kit))

        env = dict(os.environ, SC2_KIT_ROOT=str(kit))
        try:
            done = subprocess.run([str(exe), str(release)], cwd=str(work), env=env,
                                  capture_output=True, text=True, encoding="utf-8",
                                  errors="replace", timeout=300, stdin=subprocess.DEVNULL)
        except (OSError, subprocess.SubprocessError) as exc:
            return [f"无法运行 exe：{exc}"]
        if done.returncode != 0:
            problems.append(f"exe 退出码 {done.returncode}")
        if "已经是最新" not in (done.stdout or ""):
            problems.append("exe 没有报出预期结果，输出尾部："
                            + " / ".join((done.stdout or "").strip().splitlines()[-3:]))
    return problems


def main() -> int:
    sc2_console.enable_utf8_output()
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--onedir", action="store_true", help="目录式构建（启动更快，但不是单文件）")
    parser.add_argument("--no-clean", action="store_true", help="保留 PyInstaller 缓存")
    parser.add_argument("--no-smoke", action="store_true", help="跳过运行构建产物")
    args = parser.parse_args()

    if not pyinstaller_available():
        print("缺少 PyInstaller（仅构建时需要，运行时不需要）：", file=sys.stderr)
        print(f"  {sys.executable} -m pip install pyinstaller", file=sys.stderr)
        return 2

    exe = build(onefile=not args.onedir, clean=not args.no_clean)
    size = exe.stat().st_size
    digest = hashlib.sha256(exe.read_bytes()).hexdigest()
    print(f"\n产物 : {exe.relative_to(ROOT).as_posix()}")
    print(f"大小 : {size / 1048576:.1f} MB")
    print(f"SHA256: {digest}")

    if not args.no_smoke:
        problems = smoke_test(exe)
        if problems:
            print("\n冒烟测试未通过：", file=sys.stderr)
            for line in problems:
                print(f"  {line}", file=sys.stderr)
            return 1
        print("冒烟测试: 已用真实发行包运行通过")

    print("\n发给新手时，只需给这一个文件：")
    print("  1. 把它放进 kit 目录（与 tools/ 同级），或放在 kit 文件夹旁边")
    print("  2. 双击运行")
    print("  3. 把新的发行包 zip 拖进去，按提示确认")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
