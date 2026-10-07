# 自定义传输 GUI action

AeonOfIhanrii 模组库导出三个可编辑 GUI action：

| Action | 第一个输入 | 发送者与头像来源 |
|---|---|---|
| `Play Custom Transmission` | Sender（`text`） | 显式的 Sender 与 PortraitModel 输入 |
| `Play Custom Transmission By UnitType` | Unit Type（`gamelink<Unit>`） | `UnitTypeGetName` 与 `TransmissionSourceFromUnitType` |
| `Play Custom Transmission By Unit` | Unit（`unit`） | `UnitGetName` 与 `TransmissionSourceFromUnit` |

两个动作随后都接受 Message（`text`）、Audio（`soundlink`）、Wait（`bool`）、Beep（`bool`）与 Wait Duration（`fixed`，默认 2.0 秒）。Message 保持 `text`，好让触发器编辑器直接使用本地化中文文本。

每个动作发送一条带自己头像来源、所选音频、发送者与消息的传输。战役音效可能自带 `Speaker`、`Subtitle` 与 `Portrait` catalog 字段。不要先把该音效走一遍 `SendTransmissionSimple`，因为那会产生一条使用音效原始表现数据的独立传输。单位来源函数会启用 `overridePortrait`，让所选单位的头像优先于音效的 catalog 头像。

`Play Custom Transmission By Unit` 在向 CenterLeft 发送之前，先隐藏过场的 BottomLeft 头像。在 `pulnar02` 中，前面那条战役原生传输在过场模式下使用 BottomLeft，它的延迟清理跑起来之前可能还短暂可见。定点隐藏会移除那个先前的头像，而不清掉传输音频或其他头像槽位。

Audio 设为 `EditorDefaultSound` 时，Wait Duration 同时控制传输显示时长，以及（启用 Wait 时）等待时长。提供了音频时两者都用 `SoundLengthSync(Audio)`。无音频时 Wait 用真实时间，有音频时用游戏时间。Wait 关闭时两个动作都不追加尾部延时。

`Play Custom Transmission` 保留它的六输入 GUI 签名，因为战役地图在调用它。没有音频时它的显示时长是 2 秒；有音频时用 `SoundLengthSync(Audio)`。启用 Wait 时等待跟随该时长，且没有无条件的尾部延时。在 GUI `Triggers` 源里维护这些动作。不要编辑生成的 `Lib67AA1763.galaxy`。

改完组件源之后，在 SC2 编辑器里打开并保存模组。确认三个动作都保留可编辑输入并能编译。测一条带音频和不带音频的中文消息、Wait 开与关，以及一个非默认时长。对单位动作，测一个合法的既有单位实例。
