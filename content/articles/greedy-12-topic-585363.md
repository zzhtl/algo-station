---
title: 会议室扫描线：贪心训练题解
category: 贪心
summary: 会议室数量等于同时进行会议的最大数量；把开始和结束时间排序，扫描事件统计当前占用。
problem_ids: [253, 252]
order: 112
---

# 会议室扫描线：贪心训练题解

会议室 II 要求最少会议室数，本质是求任意时刻最多有多少会议重叠。

一句话记法：**开始事件占用加一，结束事件释放减一，最大占用就是答案。**

## Go 参考实现：双数组扫描

```go
func minMeetingRooms(intervals [][]int) int {
	n := len(intervals)
	starts, ends := make([]int, n), make([]int, n)
	for i, in := range intervals {
		starts[i] = in[0]
		ends[i] = in[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	rooms, endPtr := 0, 0
	for _, s := range starts {
		if s < ends[endPtr] {
			rooms++
		} else {
			endPtr++
		}
	}
	return rooms
}
```

## 为什么这样写

按开始时间扫描每个会议。如果当前开始时间早于最早结束会议，说明没有房间释放，需要新房间。否则可以复用一个已经结束的房间。

这里 `s == ends[endPtr]` 可以复用会议室，因为一个结束在 10 点，一个开始在 10 点不重叠。

## 复杂度

- 时间复杂度：$O(n \log n)$。
- 空间复杂度：$O(n)$。

## 易错点

- 相等边界写成需要新房间。
- 只按开始时间排序但不维护结束时间。
- 把 #252 能否参加所有会议和 #253 最少会议室混淆。
- 使用扫描线事件时，结束事件应在同一时间先处理。

## 练习顺序

建议按这个顺序刷：#252, #253。
