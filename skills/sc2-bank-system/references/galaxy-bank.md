# Galaxy Bank 参考

SC2 Bank native 的简明参考。

## 主要限制

要读写真正的 SC2 Bank，需要 GUI Bank 触发器

## 核心原生函数

| 函数 | 用途 |
|---|---|
| `BankLoad(name, player)` | 打开或创建一个 Bank 文件 |
| `BankSave(bank)` | 把内存中的值写到磁盘 |
| `BankSectionExists(bank, section)` | 检查 section 是否存在 |
| `BankKeyExists(bank, section, key)` | 检查键是否存在 |
| `BankValueSetFromInt/Fixed/String/Text(...)` | 存一个带类型的值 |
| `BankValueGetAsInt/Fixed/String/Text(...)` | 读一个带类型的值 |
| `BankSectionRemove(bank, section)` | 删除整个 section |
| `BankKeyRemove(bank, section, key)` | 删除单个键 |
| `BankSectionCount/Name(...)` | 遍历 section |
| `BankKeyCount/Name(...)` | 遍历 section 内的键 |

## 通用 SC2 事实

- Bank 是按玩家本地存放的 XML 文件。
- 除非用 `BankKeyExists` 守卫，缺失的键读回来是零或空值。
- `BankSave` 是必须的；写入不会自动持久化。

## 最小通用示例

```galaxy
bank b = BankLoad("ExampleBank", 1);

if (!BankKeyExists(b, "Progress", "SeenIntro")) {
    BankValueSetFromInt(b, "Progress", "SeenIntro", 0);
    BankSave(b);
}
```

## 何时仍需要直接用 Bank 原生函数

它们在以下情况仍然有用：

- 读遗留地图或旧文档
- 理解暴雪战役示例
- 在主脚本块之外编辑 GUI Bank 触发器

它们不是当前工作或新功能的主要持久化 API。
