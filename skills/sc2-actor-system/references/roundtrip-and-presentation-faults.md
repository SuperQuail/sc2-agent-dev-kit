# 编辑器往返陷阱与表现故障

## 编辑器保存之后

- 扫 `GameData/`，找出 `GameData.xml` 与带类型的旁路 catalog 之间重复的 `(catalog type, id)` 对。每个 ID 只保留一行规范行。
- 编辑器告警、空白的编辑器名、空白的命令卡图标/文本，以及预载抖动，都是提取图不完整的信号——不是无关紧要的输出。
- 重新检查子单位的带索引命令卡覆盖。编辑器会把继承的带索引 `LayoutButtons` 字段展开成本地子元素，并保留本不属于该自定义单位的继承需求。
- 编辑器保存可能把混合 catalog 对象搬到标准文件里。保留规范行，清掉同模组的重复行，保住生效字段与本地化锚点。按 ID 比对，或忽略空白比对，来找出真正的变化。

## 要剥掉的 Actor UI 钩子

当当前依赖/UI 上下文缺少被引用的 file desc 时，剥掉复制来或继承来的 Actor UI 钩子：

- `CustomUnitStatusFrame value="Coop_UnitStatus_.../..."`
- `CustomUnitStatusFrame value="HotS_UnitStatus/..."`
- `CustomUnitStatusFrame value="LotV_UnitStatus/..."`
- `StatusBarOn index="Custom" value="1"`
- `UnitFlags index="SuppressDefaultStatusBar" value="1"`

如果本地 Actor 从父级继承了有问题的状态栏，优先用 `GenericUnitStandard` 这类直接的安全父级加复制来的视觉字段，而不是保留继承的自定义 UI 链。如果当前 UI 不提供复制来的 `CustomUnitStatusFrame`，要追踪并处理该字段、`StatusBarOn`/`Custom`、`SuppressDefaultStatusBar` 与父级链——不要盲目清掉有效的 UI。然后测试训练、折跃、变形、死亡/复活、攻击与音频。

## 常见表现故障

| 症状 | 先追什么 |
|---|---|
| GenericUnitFallback / 白色球体 | 单位 Actor 创建事件、Model 的活动父级链与 `.m3`、Missile Actor、折跃技能 |
| 同一作用域里多个 `CActorUnit` | 指向原版单位的继承出生事件、多余的变形 Actor、重复的建造事件 |
| 每个效果有多个 `CActorAction` | 继承的原版效果监听器与自定义监听器同时触发 |
| 有伤害但没有弹体/光束/音效 | 武器 → Launch/Impact Effect → Action → Missile/Beam/Model/Sound |
| 保存后消失或重复 | 标准 catalog 迁移、重复定义、数组索引与父级字段规范化 |

## 来源注意事项

- **Actor 索引冲突：** [references/catalog-rules.md](../../sc2-catalog-xml/references/catalog-rules.md) 的 Ravager 表（索引 4/5 与裸 `UnitConstruction` term）与 [references/editor-roundtrip-and-actors.md](editor-roundtrip-and-actors.md) 的 Start/Finish 规则不一致。一律检查实时父 Actor 的 `On` 数组；不要照抄固定索引表。
- **音效导出冲突：** catalog 规则声称不存在活动的音效转储，但 `DataEditorXML/` 里实际含有分层 `Sounds.txt`。以真实文件和依赖启用状态为准。
