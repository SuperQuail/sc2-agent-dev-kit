# 六阶段问题生命周期（强制）

每个上报的 bug 或缺陷都必须走：

```text
reported → root cause confirmed → source fixed → static validation passed → Editor accepted → packaged runtime passed
```

1. **`reported`**：记录到 `sc2agent-records/<project>/issues.md`，含：地图/任务、玩家/阵营配置、观察到的与预期的行为、确切的编辑器告警或游戏内错误（来自 `bugreport.txt`）。
2. **`root cause confirmed`**：定位到根因 catalog ID、父级继承缺陷或 Galaxy 逻辑错误。
3. **`source fixed`**：工作区源文件已更新。
4. **`static validation passed`**：`python tools/test-suite.py` 零错误跑完。
5. **`Editor accepted`**：SC2 编辑器能干净地打开改动后的模组/地图——没有 XML schema 告警、依赖缺失错误或触发器编译失败。
6. **`packaged runtime passed`**：改动已在游戏内测试并验证。

> 静态测试套件跑干净 **不能** 让问题越过编辑器闸门。

## 账本维护

- 当前账本：`sc2agent-records/<project>/issues.md`
- 保持当前账本精简。
- 长期系统事实移到主题正页，不要留在冗长的历史讨论里。
- 用 `python tools/sc2.py issues` 校验账本。
