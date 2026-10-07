# 控制流与语言陷阱

Galaxy 接受的语句形式，以及最常打断脚本的编译器规则。

## 控制流

```galaxy
// if / else if / else
if (gf_IsSurvival() == true) {
    // ...
}
else if (lv_raceCheck == true) {
    // ...
}
else {
    // ...
}

// for loop (Galaxy compiler pattern – uses auto variables):
const int auto4853DB8A_ai = 1;
int auto4853DB8A_ae = gv_spawnerLastIndex;
lv_index = 0;
for ( ; ( (auto4853DB8A_ai >= 0 && lv_index <= auto4853DB8A_ae)
         || (auto4853DB8A_ai < 0 && lv_index >= auto4853DB8A_ae) )
      ; lv_index += auto4853DB8A_ai ) {
    // body
}

// while (PlayerGroup iteration pattern):
lv_loopedPlayer = -1;
while (true) {
    lv_loopedPlayer = PlayerGroupNextPlayer(autoGroup, lv_loopedPlayer);
    if (lv_loopedPlayer < 0) { break; }
    // body
}
```

---

## 常见陷阱与语言规则

以下是写 Galaxy 时最常见的错误 —— 来源是官方 SC2 编辑器指南与 SC2Mapster 社区：

| 规则 | 说明 |
|---|---|
| 变量声明在顶部 | 所有局部变量**必须**在函数最顶部声明 —— 位于任何语句、条件或函数调用之前。编译器严格执行。 |
| 无 `++` / `--` 运算符 | `var++` 与 `var--` 不是合法语法。用 `var += 1` 和 `var -= 1`。 |
| 无块注释 | Galaxy 没有 `/* */` 块注释。只用 `//` 行注释。 |
| `static` = 文件私有 | `static` 函数在自己的 `.galaxy` 文件之外不可见。用它避免污染全局命名空间。 |
| 行长度上限 | 行不得超过 2048 个字符（编译器硬限制）。 |
| 跨文件调用无需 include | 两个文件都 include 进 `MapScript.galaxy` 之后，任一文件都能调用另一个里定义的函数 —— 调用方文件顶部**不需要**局部 `include`。所有被 include 的文件共享同一个全局编译单元。 |
| 空主体的 `for` 循环 | Galaxy 的 `for` 循环可以有空主体；迭代步进照常执行。有时用它做跳过模式。 |
| `fixed` 是 20 位 | `fixed` 是**定点**类型（不是 IEEE 浮点）。精度约 0.0001，最大值约 524287。SC2 说 "Real" 的地方一律用它。 |
| 无动态内存 | Galaxy 没有 `new`/`delete`，也没有堆分配。存储只有静态全局、栈上局部，或引擎管理的句柄（`unit`、`region`、`timer` 等）。 |
| `TriggerCreate` 里的函数名 | `TriggerCreate` 接收函数的**字符串名** —— 不是 funcref。函数签名必须是 `bool (bool, bool)`。例：`TriggerCreate("MyHandler_Func")`。 |

```galaxy
// switch-style (auto val):
int auto4221B408_val = lv_race;
if (auto4221B408_val == 1) { PlayerSetRace(lp_player, "Terr"); }
else if (auto4221B408_val == 2) { PlayerSetRace(lp_player, "Zerg"); }
else if (auto4221B408_val == 3) { PlayerSetRace(lp_player, "Prot"); }
```
