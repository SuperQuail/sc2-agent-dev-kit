# 排障 `Scri: Script load failed: Function not found`

函数明明在磁盘上、加载时却解析不到 —— 原因是编码，以及修法。

## 常见错误："Scri: Script load failed: Function not found"

如果 SC2 编辑器或测试地图抛出：

```
Scri: Script load failed: Function not found
```

而函数确实存在于你的 `.galaxy` 文件里，最常见的原因是**文件编码**。

**修法：** 把出问题的 `.galaxy` 文件存成**不带 BOM 的 UTF-8**（不是「带 BOM 的 UTF-8」）。

在 VS Code 里：
1. 打开文件。
2. 点右下角状态栏的编码指示（例如 `UTF-8 with BOM`）。
3. 选 **"Save with Encoding"** -> 选 **"UTF-8"**（无 BOM）。

SC2 引擎无法解析带 BOM（`EF BB BF`）前缀的文件，因此找不到该文件里定义的任何函数，哪怕代码本身是对的。
