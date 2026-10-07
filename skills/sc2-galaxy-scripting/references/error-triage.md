# 社区错误分诊

| 错误 | 原因 | 修法 |
|---|---|---|
| `Can only pass basic types` | 把结构体/数组当函数参数 | 传标量 ID/索引，或改用全局变量 |
| `struct forward declaration not supported` | 结构体在定义前被使用 | 把结构体定义放在使用它的函数/变量之前 |
| `Bulk copy not supported` | 按值赋数组/结构体 | 显式逐个复制标量元素 |
| `Could not allocate Global Memory` / `e_globalsTooLarge` | 全局数组太大 | 让项目状态保持紧凑 |
| `failed: 32k - 1 size limit to local variables` | 巨大的局部数组 | 改用小全局变量或拆分 |
| `Registry overflow` | 单个表达式里 string/text/point 引用太多 | 拆成更小的语句 |
| `Internal compiler error` | 行或字符串超过约 2046 字符 | 拆开长行/长字符串 |
