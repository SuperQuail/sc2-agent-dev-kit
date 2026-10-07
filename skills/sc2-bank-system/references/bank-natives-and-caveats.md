# 核心原生函数

| 函数 | 用途 |
|---|---|
| `BankLoad(name, player)` | 打开或创建一个 Bank 文件 |
| `BankSave(bank)` | 把内存中的值写到磁盘（写入不会自动持久化） |
| `BankSectionExists(bank, section)` | 检查 section 是否存在 |
| `BankKeyExists(bank, section, key)` | 检查键是否存在 |
| `BankValueSetFromInt/Fixed/String/Text(...)` | 存一个带类型的值 |
| `BankValueGetAsInt/Fixed/String/Text(...)` | 读一个带类型的值（键不存在时返回空/零默认值） |
| `BankSectionRemove(bank, section)` | 删除整个 section |
| `BankKeyRemove(bank, section, key)` | 删除单个键 |
| `BankSectionCount/Name(...)` | 遍历 section |
| `BankKeyCount/Name(...)` | 遍历 section 内的键 |

## Bank API 命名注意事项

`references/bank-system.md` 的例子用 `BankValueGetInt` / `BankValueSetInt` 和单参数的 `BankPreLoad`。[references/galaxy-bank.md](galaxy-bank.md) 与 `skills/sc2-map-triggers/references/triggers-native/b.md` 里的原生表用的是 `BankValueGetAsInt` / `BankValueSetFromInt`，而 `BankPreload(name, player)` 接受两个参数。不要照抄那个例子——保留架构，但用正确的原生名。

把 GUI Bank 接线与实际生成的脚本放在一起交叉核对。不要把旧项目「Bank API 只能走 GUI」的结论当成通用引擎限制。

完整原生签名：[references/galaxy-bank.md](galaxy-bank.md)。

## schema 迁移注意事项

迁移旧 schema 版本时要保住已获得的进度。不要把示例里的重置盲目套到完成状态上。实际玩家、Bank 名、预载阶段与文件路径来自项目和运行环境——不要硬编码。
