# 发行规范与安装器

本文件约定 StarCraftIIAgent 的**发布产物形态**、**安装布局**与**版本规则**。
打包脚本 `tools/release.py` 与安装器 `installer/` 都以本文件为准。

---

## 0. 安装器是 Rust 写的

第一版用 Electron，产物 **70.88 MB**，而应用本体只有 **0.07 MB**——99.9% 是运行时。
更糟的是安装逻辑跑在同步 IPC 处理器里，**界面会僵住、看不到进度**。

现在改用 Rust + egui：

| | Electron 版 | Rust 版 |
|---|---|---|
| 单文件体积 | 70.88 MB | **5.53 MB** |
| 运行时依赖 | Chromium + Node | 无 |
| 安装时界面 | 卡死，无进度 | 后台线程，逐文件进度 |

关键设计：安装全程跑在 `std::thread` 里，通过 `mpsc` 把进度送回 UI，
并调用 egui 的 `request_repaint` 唤醒重绘。**引擎不持有 egui 类型**，
所以同一套逻辑也能给命令行用。

界面视觉参考 Koishi 的暗色风格（深蓝紫底、圆角卡片、紫色强调色），布局是自订的线性安装向导。

## 1. 为什么拆成两个包

套件里 `DataEditorXML/` 是约 **166 MB** 的游戏数据导出，其余全部内容只有约 **10 MB**。
放在同一个归档里意味着**每次更新一个错别字都要重下 166 MB**。

拆开后安装器能在解压前比哈希：**数据包没变就整体跳过，一个字节都不重写。**

```text
                   未压缩      压缩后     需要重下？
core  套件本体      10.2 MB     1.29 MB   仅在改动时
data  游戏数据     166.3 MB    18.71 MB   哈希一致即跳过
```

实测：完整安装（含游戏数据）约 30 秒；数据未变时 **2.9 秒**，166.3 MB 一个字节未重写。

## 2. 产物形态

发布目录 `dist/release/<version>/` 下三个文件，命名固定：

```text
StarCraftIIAgent-<version>-core.zip        套件本体（tools/ skills/ AGENTS.md …）
StarCraftIIAgent-<version>-data.zip        DataEditorXML/
StarCraftIIAgent-<version>-manifest.json   全部文件 SHA-256 + 两个包的摘要
```

**core.zip 内自带一份 `sc2agent-release.json`。** 用户常常只从 GitHub release 下这一个文件，
它必须能自我描述，否则安装器不知道数据包叫什么、哈希是多少。

内嵌那份**不含 core.zip 自己的哈希**（归档无法包含自身摘要），外层 manifest 才含。

## 3. 版本规则

版本号**只有一处真源**：`tools/sc2_version.py` 的 `VERSION`。
打包器写进归档名、内嵌描述、外层清单；安装器写进 `install.json`。**不要在别处再写一份。**

| 字段 | 何时递增 |
|---|---|
| `VERSION` | 任何内容变化 |
| `LAYOUT_REVISION` | 安装布局变了（技能结构、产物命名、清单字段语义） |

## 4. 安装布局

```text
%LOCALAPPDATA%\sc2agent\            安装根（可用 --root 改）
  kit\                              套件只装一份
  install.json                      版本、产物摘要、已链接的 harness

<harness 配置目录>\skills\<技能名>\   每个 harness 一份副本
  SKILL.md                          注入套件路径（在 frontmatter 之后）
  references\
```

**为什么拍平**：实测四个 harness 的约定都是 `<skills-dir>/<name>/SKILL.md`；
Codex 的嵌套只出现在 `.system/` 这个特殊目录。套件 29 个技能名**互不重复**，拍平无碰撞。

**为什么注入在 frontmatter 之后**：harness 把开头的 `---` 块当 YAML 解析，
任何前置文本都会**静默破坏**该技能。

## 5. 支持的 harness

| id | 名称 | 技能目录 |
|---|---|---|
| `dsh` | DeepSeek Harness | `~/.dsh/skills/` |
| `claude` | Claude Code | `~/.claude/skills/` |
| `codex` | Codex | `~/.codex/skills/` |
| `opencode` | opencode | `~/.config/opencode/skills/` |

检测依据是**配置根目录存在**（如 `~/.dsh`），`skills/` 子目录按需创建。
新增 harness 只需在 `installer/src/harness.rs` 加一行。

**界面上默认一个都不勾选**，由用户自己决定交付到哪里。

## 6. 用法

```bash
installer\target\release\sc2agent-installer.exe            # 图形界面
sc2agent-installer.exe --from <来源>                        # 预填来源
sc2agent-installer.exe --from <来源> --harness dsh --run    # 预填并直接开始

sc2agent-installer.exe --cli plan    --from <来源>          # 只报告差异
sc2agent-installer.exe --cli install --from <来源> --harness dsh,claude,codex,opencode
```

来源三种形式：

| 形式 | 例子 |
|---|---|
| GitHub release | `owner/repo`、`owner/repo@v0.1.0a1`、release 页面 URL |
| 本地发行目录 | `dist/release/0.1.0a1` |
| 本地 zip | `…-core.zip`（读内嵌描述） |

从 GitHub 拉取时国内通常要加代理：`--proxy http://127.0.0.1:7897`，
或设环境变量 `SC2AGENT_PROXY`（图形界面会预填）。下载走 `curl.exe`（Windows 10+ 自带）。

## 7. 构建

```bash
cd installer
cargo build --release        # 产物 target/release/sc2agent-installer.exe
```

首次构建会拉约 295 个 crate。`.cargo/config.toml` 已把源换成 rsproxy 稀疏索引（国内直连 crates.io 很慢）。
体积优化在 `Cargo.toml` 的 `[profile.release]`。

## 8. 发布流程

```bash
# 1. 改 tools/sc2_version.py 的 VERSION，并同步 installer/Cargo.toml 的 version
# 2. python tools/release.py --write
# 3. cd installer && cargo build --release && cd ..
# 4. 四个文件一起传（含 exe），tag 用 v<version>
```

> **exe 必须进 release。** 0.1.0a1 首发时漏了它——命令行用户有 core.zip 可用，
> 但非技术用户拿到 core.zip 无从下手，而 exe 才是双击即用的入口。
> `release.py` 是纯 Python 的，构建 exe 需要 Rust 工具链，两者刻意分开；
> 代价就是这一步必须手动记住。

## 9. 已验证（本机实测，未打开任何 harness）

```text
检测    dsh / claude / codex / opencode 四个全部识别
安装    4 × 29 个技能，505 步，2.9 秒
进度    逐文件粒度，界面全程可响应
注入    位于 frontmatter 之后，YAML 未被破坏
跳过    数据包哈希一致则整体跳过，166.3 MB 未重写
CLI     plan 与 install 均通过，输出可重定向
```

## 10. 已知限制

- **exe 未做代码签名**，Windows 首次运行会提示未知发布者
- 界面中文字体从系统加载（`msyh.ttc` 等）；系统无 CJK 字体时中文会显示为方块
- 只验证了 Windows；`curl.exe` 与 `AttachConsole` 都是 Windows 路径
