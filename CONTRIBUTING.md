# 贡献指南

## 分支策略

```text
main    发布分支。只接收来自 dev 的合并，合并后打 tag 发 release。
        不在 main 上直接开发。

dev     开发分支。日常开发都在这里。

其他     临时功能分支，从 dev 切出，合回 dev。
```

### 一次改动的流转

```bash
git switch dev
git pull
# ... 改动 ...
python -m pytest tools/tests -q        # 本地先过一遍
python tools/sc2.py check .
git commit -m "..."
git push origin dev

# 准备发布时
git switch main
git merge --ff-only dev               # 只允许快进，保证 main 干净
git push origin main
git tag v0.1.0a2 && git push origin v0.1.0a2
```

**为什么 `--ff-only`**：它会让有分叉的合并失败，而不是悄悄生成一个合并提交。
发布分支的历史应当是一条直线，这样 `git log main` 直接就是版本清单。

## 提交前必须过的门禁

```bash
python -m pytest tools/tests -q          # 单元测试
python tools/test-suite.py --scope tools # 预飞行契约
python tools/audit-skill-frontmatter.py  # 技能 frontmatter
python tools/check-doc-links.py          # 文档断链
python tools/sc2.py check .              # 静态规则
```

这五条正是 CI 跑的内容（`.github/workflows/ci.yml`）。本地过了 CI 才会过。

## CI 为什么跑在 Windows 上

本套件驱动 SC2 编辑器，会调用 PowerShell 与 `curl.exe`，并分发 `.exe`——
SC2 模组开发本身就是 Windows 工作流。测试里的路径处理针对 `E:/Games/StarCraft II/Mods`
这类 Windows 盘符路径，在 Linux 上 `Path.resolve()` 会把它当成相对路径拼接，
于是所有路径断言都会以真实用户不可能遇到的方式失败。

## 版本号

改版本号只动一处：`tools/sc2_version.py` 的 `VERSION`。
归档名、发行清单、安装记录都从它取值。改了内容就递增 `VERSION`；
改了发布布局（目录结构、产物命名、清单字段语义）再递增 `LAYOUT_REVISION`。
