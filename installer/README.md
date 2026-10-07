# 发行规范与安装器

本文件约定 StarCraftIIAgent 的**发布产物形态**、**安装布局**与**版本规则**。
打包脚本 `tools/release.py` 与安装器 `installer/` 都以本文件为准。

---

## 1. 为什么拆成两个包

套件里 `DataEditorXML/` 是约 **166 MB** 的游戏数据导出，其余全部内容只有约 **10 MB**。
放在同一个归档里意味着**每次更新一个错别字都要重下 166 MB**。

拆开后安装器能在解压前比哈希：**数据包没变就整体跳过，一个字节都不重写。**

```text
                   未压缩      压缩后     需要重下？
core  套件本体      10.1 MB     1.2 MB    仅在改动时
data  游戏数据     166.3 MB    18.7 MB    哈希一致即跳过
```

实测：首次安装约 30 秒；第二次 **6.8 秒**，166.3 MB 数据包一个字节未重写。

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

### 清单结构

```json
{
  "name": "StarCraftIIAgent",
  "version": "0.1.0",
  "layout_revision": 1,
  "artifacts": {
    "core": { "file": "...-core.zip", "sha256": "...", "required": true },
    "data": { "file": "...-data.zip", "sha256": "...", "required": false }
  },
  "files": { "core": { "<相对路径>": "<sha256>" }, "data": { ... } },
  "skills": [ { "name": "galaxy-units-and-groups",
                "source": "skills/galaxy/galaxy-units-and-groups",
                "group": "galaxy" } ]
}
```

`skills` 是安装的关键：harness 要求**扁平**布局，而套件把 19 个子技能嵌套在
`skills/galaxy/` 与 `skills/sc2data/` 下。安装器按 `name` 拍平，从 `source` 复制。

## 3. 版本规则

版本号**只有一处真源**：`tools/sc2_version.py` 的 `VERSION`。
打包器写进归档名、内嵌描述、外层清单；安装器写进 `install.json`。**不要在别处再写一份。**

| 字段 | 何时递增 |
|---|---|
| `VERSION` | 任何内容变化 |
| `LAYOUT_REVISION` | 安装布局变了（技能结构、产物命名、清单字段语义） |

已装版本的 `layout_revision` 低于当前，说明**旧安装无法就地升级**，需重装。

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
任何前置文本都会**静默破坏**该技能。那句注入告诉 agent 套件装在哪，
否则文档里的 `python tools/sc2.py` 命令无从解析。

## 5. 支持的 harness

| id | 名称 | 技能目录 |
|---|---|---|
| `dsh` | DeepSeek Harness | `~/.dsh/skills/` |
| `claude` | Claude Code | `~/.claude/skills/` |
| `codex` | Codex | `~/.codex/skills/` |
| `opencode` | opencode | `~/.config/opencode/skills/` |

检测依据是**配置根目录存在**（如 `~/.dsh`），`skills/` 子目录按需创建。
新增 harness 只需在 `installer/lib/harness.js` 加一行。

## 6. 用法

### 图形界面

```bash
cd installer && npm install && npm start
```

界面四步：填来源 → 勾选 harness → 看差异预览 → 安装。**不会打开任何 harness。**

### 命令行（无界面，可脚本化）

```bash
node installer/cli.js detect                                    # 检测 harness
node installer/cli.js plan   --from <来源>                      # 只报告差异
node installer/cli.js install --from <来源>                     # 实际安装
node installer/cli.js install --from <来源> --harness dsh,codex # 限定目标
node installer/cli.js install --from <来源> --dry-run           # 什么都不写
```

来源三种形式：

| 形式 | 例子 |
|---|---|
| GitHub release | `owner/repo`、`owner/repo@v0.1.0`、release 页面 URL |
| 本地发行目录 | `dist/release/0.1.0` |
| 本地 zip | `…-core.zip`（读内嵌描述） |

## 7. 发布流程

```bash
# 1. 改 tools/sc2_version.py 的 VERSION
# 2. python tools/release.py --write
# 3. 把 dist/release/<version>/ 下三个文件传到 GitHub release，tag 用 v<version>
```

发布前自检（`release.py` 会拒绝打包 `agent-config.json`，那是机器专属配置）：

```bash
python tools/sc2.py check .
python tools/audit-skill-frontmatter.py
python tools/check-doc-links.py
python -m pytest tools/tests -q
```

## 8. 已验证（本机实测，未打开任何 harness）

```text
检测    dsh / claude / codex / opencode 四个全部识别
安装    4 × 29 个技能，每个都带 SKILL.md + references/
注入    位于 frontmatter 之后，YAML 未被破坏
跳过    二次安装 6.8 秒，166.3 MB 数据包一个字节未重写
识别    DeepSeek Harness 实时读取技能目录，29 个技能全部出现
```
## 9. 两个环境坑（实测踩到）

### `ELECTRON_RUN_AS_NODE` 必须清掉

该变量会让 `electron.exe` 退化成普通 Node，于是 `require('electron')`
返回的是**路径字符串**而不是 API，界面启动即崩：

```
TypeError: Cannot read properties of undefined (reading 'handle')
    at Object.<anonymous> (main.js:70:9)
```

本机环境就设了这个变量。用 `start.cmd` 启动（它会临时清掉），或手动：

```bash
set ELECTRON_RUN_AS_NODE=
npx electron .
```

### npm 会拦下 Electron 的 postinstall

npm 的 allow-scripts 策略默认不跑 `electron` 的 postinstall，结果是**包装好了但没有二进制**。
补跑一次即可：

```bash
cd node_modules/electron && node install.js
```

国内网络可先设镜像：`set ELECTRON_MIRROR=https://npmmirror.com/mirrors/electron/`

## 10. 构建 exe

两套方案，实测都能用：

### 单文件便携版（推荐分发）

```bash
cd installer && npm install
npx electron-builder --win portable --config.win.signAndEditExecutable=false
```

产物 `dist-portable/SC2Agent-Installer-0.2.0.exe`，**单文件 70.9 MB**，双击即用。

> **`--config.win.signAndEditExecutable=false` 不是可选项。** 不加它会去下载 `winCodeSign`，
> 那个包里含 **macOS 的符号链接**（`darwin/10.12/lib/libcrypto.dylib`），Windows 上创建符号链接需要
> 开发者模式或管理员权限，于是构建报 `Cannot create symbolic link` 并重试到失败。
> 代价是 exe 不带版本元数据与图标——对内部工具无影响。

### 目录版（便于改 UI 后快速重打包）

```bash
npx @electron/packager . SC2Agent-Installer --platform=win32 --arch=x64 --out=dist-exe --overwrite --prune=true
```

产物 `dist-exe/SC2Agent-Installer-win32-x64/SC2Agent-Installer.exe`。
该目录里 180 MB 是 Electron 运行时，`resources/app.asar` 只有 **0.07 MB**——那才是我们写的代码。

### 启动前必须清掉的环境变量

两种形态都受这一条影响，见第 9 节。双击 `.exe` 不受影响（不继承该变量），
但从终端启动时要：`set ELECTRON_RUN_AS_NODE=`。

### 冒烟测试的差异

| 形态 | `--smoke` 可用 |
|---|---|
| 目录版 exe | ✅ 直接可用，验证 main / preload / IPC / 渲染层 |
| 便携版 exe | ❌ NSIS 自解压壳会另起子进程，argv 与 stdio 都不透传 |

便携版改用窗口检测验证：进程出现且 `MainWindowTitle` 非空。实测 **4.1 秒**出窗口。
