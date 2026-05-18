---
title: 计数差分判断覆盖：哈希与前缀训练题解
category: 哈希与前缀
summary: 覆盖关系常能转成计数差分；进入事件加一，离开事件减一，扫描时维护当前覆盖次数。
problem_ids: [1094, 1109, 732]
order: 110
---

# 计数差分判断覆盖：哈希与前缀训练题解

区间批量加减适合差分。只关心某一时刻覆盖次数时，也可以把区间端点变成事件，用有序扫描维护当前值。

## Go 参考实现：拼车

```go
func carPooling(trips [][]int, capacity int) bool {
	diff := make([]int, 1001)
	for _, t := range trips {
		passengers, from, to := t[0], t[1], t[2]
		diff[from] += passengers
		diff[to] -= passengers
	}
	cur := 0
	for _, x := range diff {
		cur += x
		if cur > capacity {
			return false
		}
	}
	return true
}
```

## 为什么这样写

乘客在 `from` 上车，在 `to` 下车，影响区间是 `[from, to)`。差分在起点加，在终点减，扫描前缀和就是每个位置车上的人数。

如果坐标范围很大，要用有序 map 或排序事件，而不是开巨大数组。

## 易错点

- 右端点开闭弄错，`to` 处乘客已经下车。
- 坐标范围大还开数组。
- 扫描时只看事件点，但中间值没有变化，所以事件点足够。
- 差分数组没有多开或边界越界。

## 练习顺序

建议按这个顺序刷：#1109, #1094, #732。
