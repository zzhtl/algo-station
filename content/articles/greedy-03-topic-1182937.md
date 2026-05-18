---
title: 加油站总量与最低点：贪心训练题解
category: 贪心
summary: 加油站先看总油量是否够；若从 start 到 i 油箱变负，start 到 i 之间都不可能作为起点。
problem_ids: [134]
order: 103
---

# 加油站总量与最低点：贪心训练题解

加油站题有两个判断：总油量是否足够，以及从哪里开始不会中途断油。

一句话记法：**总量不够无解；局部油箱掉负，就从下一站重新开始。**

## Go 参考实现

```go
func canCompleteCircuit(gas []int, cost []int) int {
	total, tank, start := 0, 0, 0
	for i := 0; i < len(gas); i++ {
		diff := gas[i] - cost[i]
		total += diff
		tank += diff
		if tank < 0 {
			start = i + 1
			tank = 0
		}
	}
	if total < 0 {
		return -1
	}
	return start
}
```

## 为什么这样写

如果从 `start` 出发，到 `i` 时累计油量变负，那么 `start..i` 中任意一个站作为起点都不可能成功。因为从中间站出发只会少拿前面那段曾经积累的油，不会更好。

所以这整段候选可以一次性排除，下一个候选起点是 `i+1`。

总量 `total` 决定是否存在解；局部 `tank` 决定候选起点如何推进。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(1)$。

## 易错点

- 只看局部 tank，没有判断总量。
- tank 变负后 start 没设成 `i+1`。
- tank 重置后 total 也清零，导致总量判断错误。
- 试图从每个起点模拟一圈，复杂度 $O(n^2)$。

## 练习顺序

建议先刷 #134。
