# 发行规范与安装器

本文件约定**开发套件安装器**（`installer/`）的行为，以及套件的**发布产物形态**、
**安装布局**与**版本规则**。打包脚本 `tools/release.py` 也以本文件为准。

安装器名字里不带 SC2：它装的是「套件」这个抽象，SC2 Agent Dev Kit 只是当前接入的第一个。
套件的技能列表、目标目录、包体哈希都由发布方的 manifest 描述，接新套件不需要改安装器。

---

## 0. 前端是 Tauri + React，后端是 Rust

安装器经历了两代：

| | Electron | Rust + egui | **Tauri + React** |
|---|---|---|---|
| 单文件体积 | 70.88 MB | 5.59 MB | **3.14 MB** |
| 界面技术 | 手写 HTML/CSS/JS | 立即模式绘制 | **React 19 + antd 6** |
| 运行时依赖 | Chromium + Node | 无 | 系统 WebView2 |
| 安装时界面 | 卡死，无进度 | 后台线程，逐文件进度 | 同左 |

### 为什么从 egui 换成 Tauri

egui 是立即模式绘制：每个控件的位置、间距、换行都要自己算。代价是
一个 Grid 里写 set_width(available_width()) 完全无效、文字被挤成一列一个字；
卡片宽度要手工 set_min_width；中文字体得当独立问题处理。

换成 WebView2 + React + antd 之后：

* 控件用现成的（Input / Button / Checkbox / Card / Progress / Alert），不再手搓；
* 布局交给 CSS flex，不再手工算坐标；
* **中文字体问题消失** —— 浏览器自己做字体回退，不存在豆腐块；
* 体积反而更小，因为不往二进制里塞渲染引擎。

### 结构

```text
installer/
  src-tauri/          Rust 后端
    src/main.rs       入口：命令行开关分派，否则起 Tauri
    src/commands.rs   Tauri 命令层（前端能调的全部入口）
    src/install.rs install_engine.rs source.rs net.rs mirror.rs
    src/selfupdate.rs harness.rs manifest.rs version.rs cli.rs
  ui/                 React 前端
    src/App.tsx       界面
    src/api.ts        Tauri invoke / event 封装与类型
    src/theme.ts      antd 主题令牌（Koishi 暗色：深蓝紫底、紫色强调）
```

### 安装引擎一行没改

install_engine 仍然往 mpsc 里送 Msg，commands.rs 把同一条流转发成 Tauri 事件
（install://progress）。少一层耦合，也少一处会和 CLI 行为分叉的地方。

### 命令线仍然在

同一个 exe：不带命令行开关启动图形界面，带 --cli / --self-check 等走无界面模式，
两者共用同一套引擎。
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
pnpm -C ui install
ui/node_modules/.bin/tauri build --no-bundle    # 产物 src-tauri/target/release/devkit-installer.exe
```

几点注意：

* **必须从 installer/ 调 tauri**，不是从 ui/：CLI 按约定在**子目录**里找 src-tauri，
  从 ui/ 跑会报 “Couldn't recognize the current folder as a Tauri project”。
* 加 --no-bundle 只出 exe，跳过 NSIS 打包（那一步要去 GitHub 下载 nsis-3.13.zip，国内常卡住）。
* 生产构建必须开 custom-protocol 特性（Cargo.toml 里已声明）。不开的话 release 版会去连
  devUrl，在没有 dev server 的机器上就是一片 ERR_CONNECTION_REFUSED。
* 前端构建产物内嵌进 exe，所以改了 ui/ 要重新跑 tauri build（它会先自动跑 pnpm -C ../ui build）。

### 网络

**cargo 和 pnpm 对代理的偏好相反**，实测：

| | 结论 |
|---|---|
| cargo | **要挂代理**（HTTP_PROXY/HTTPS_PROXY）。不挂时 rsproxy 的连接会被掐断，刷 “server closed abruptly” |
| pnpm | **不要挂代理**。官方源直连 7 MB / 20s；挂上代理 25s 只下了 1/12 |

crates 的镜像见 src-tauri 上层 .cargo/config.toml。若某个 .crate 被截断，cargo 会报
“failed to unpack ... unexpected end of file”；按索引里的 cksum 重下那一个即可。
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

## 10. 中文字体

egui 自带字体只有拉丁字母，界面又要显示中文——所以启动时必须找一个 CJK 字体。

### 找法

1. `SC2AGENT_FONT`（`路径` 或 `路径#面序号`）——显式指定，最高优先级
2. 自动搜索 `%WINDIR%\Fonts` 与 `%LOCALAPPDATA%\Microsoft\Windows\Fonts`
3. 按文件名优先级排序：中文版 Windows 默认字体 → 常见第三方中文字体（Noto / 思源 / 更纱 / MiSans / 鸿蒙）→ 其他 CJK（繁体、日文、韩文）
4. **逐个验证字形覆盖，命中即停**——不做无谓的全盘扫描

### 两个坑

**文件名不可信。** 本机 568 个字体里，用 `deng` 匹配「等线」时把 `OLDENGL.TTF`
（一种英文花体）也匹配进来了。所以每个候选都要真的解析 cmap，确认能画出「中」；
判定用四个简繁共用字（中文的一）+ 简体「设」或繁体「設」二选一，简繁字体都能通过。

**全盘扫描太慢。** 568 个字体文件、动辄十几 MB，全读一遍要好几秒。
所以先只在排名靠前的十余个文件里找，全部失败才退化成扫描整个目录。

### 系统真的没有 CJK 字体时

界面顶部会出现一条**英文**提示（中文提示自己也渲染不出来）：

```
No Chinese-capable font found on this system
The interface is Chinese-only and cannot be rendered. Install any CJK font ...
or point SC2AGENT_FONT at a font file you already have.
The command line still works: --cli plan|install --from <source>
```

中文显示为方块，但界面其余部分（路径、按钮、harness 名）仍可用，
命令行模式完全不受影响。

### 排查

```bash
sc2agent-installer.exe --font-probe
```

会打印搜索了哪些目录、选中了哪个文件、是自动发现还是 `SC2AGENT_FONT` 指定的。

## 11. 网络：镜像与代理

移植自 HSCL（`D:\Code\Rust\HSCL`）的 update 模块。

### 镜像竞速

国内直连 `github.com` 的 release 资产经常几十 KB/s 甚至超时。所以包体下载会把一个 URL
展开成「直连 + 若干反代镜像」的候选列表，**并发竞速，谁先完成用谁，其余立刻掐掉**：

```text
直连                              ← 海外用户直接命中
https://gh.ddlc.top/https://…      ← 国内镜像，按实测速度排序
https://gh-proxy.com/https://…
https://ghfast.top/https://…
https://cors.isteed.cc/https://…
https://ghproxy.cc/https://…
https://github.akams.cn/https://…
```

每个候选写自己的 `.partN`，赢家改名落盘；失败信息一并保留，
方便告诉用户「直连超时、镜像 403」之类。实测一轮竞速 7 个地址、0.5 MB 清单走直连完成。

只对 **GitHub 的 URL** 展开镜像；清单这类小文件不竞速（为 0.5 MB 开七个进程不值得）。
用 `--mirror off` 或 `SC2AGENT_MIRROR=off` 关掉。

### 代理自动探测

顺序：**环境变量 → Windows 系统代理 → 直连**。

环境变量认 `HTTPS_PROXY` / `https_proxy` / `ALL_PROXY` / `HTTP_PROXY` / `http_proxy`（大小写都试）；
系统代理读注册表 `Internet Settings` 的 `ProxyEnable` / `ProxyServer`，
并处理 `http=a:1;https=b:2` 这种分协议写法。`host:port` 会自动补成 `http://host:port`。

## 12. 自更新

安装器会自己检查新版并就地替换。界面启动时后台查一次（失败静默，用户没要求检查就不打扰）；
命令行用 `--self-check` 与 `--self-update`。

### 找版本时不能用 /releases/latest

那个接口**按定义跳过预发布**，而本项目的版本全是 `a` 预发布。结果是它 404、
重定向落到 `/releases` 列表页，tag 被解析成字符串 `"releases"`（亲眼见过）。
改成读发布列表 API（`?per_page=30`），按版本号排序取最高的；API 不可用时再抓 releases 页面。

### 替换手法

Windows **允许重命名正在运行的 exe**，所以不需要额外的批处理脚本：

```text
1. 新版下到 <exe>.new
2. 自己改名成 <exe>.old
3. .new 改名到原路径
4. 启动新进程，自己退出；下次启动时顺手删掉 .old
```

比「写个 cmd 等进程退出再替换」少一个会失败的环节。下载后会校验是 PE 文件（`MZ` 开头）——
镜像站返回一个 HTML 错误页是最常见的失败形态，直接改名会把安装器换成一段 HTML。

放在 `Program Files` 等受保护目录时会改名失败，此时会明确提示手动下载替换。

### 版本比较

两种后缀写法视为同一个版本：`0.1.0-a1`（Cargo.toml，semver 要求连字符）与 `0.1.0a1`（`tools/sc2_version.py`，也出现在 git tag 上）。
数字段先比；相同则预发布后缀按字符串比（`a1 < a2 < b1`）；有后缀的比无后缀的旧。

用 `SC2AGENT_NO_SELF_UPDATE=1` 关掉启动时的自动检查。
## 13. 已知限制

- **exe 未做代码签名**，Windows 首次运行会提示未知发布者
- 只验证了 Windows；`curl.exe` 与 `AttachConsole` 都是 Windows 路径
- 字体验证只抽查 5 个汉字。理论上存在「能画这 5 个字但缺其他字」的字体，实际几乎不可能