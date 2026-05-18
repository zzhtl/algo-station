---
title: 任务调度冷却槽：贪心训练题解
category: 贪心
summary: 任务调度的下界由最高频任务决定；先摆放最高频任务形成冷却槽，再看其他任务能否填满空位。
problem_ids: [621]
order: 108
---

# 任务调度冷却槽：贪心训练题解

任务调度器的关键是最高频任务。它们之间必须隔开 `n` 个冷却时间，所以会形成最少时间的骨架。

一句话记法：**最高频任务决定框架，同最高频个数决定最后一列宽度。**

## Go 参考实现

```go
func leastInterval(tasks []byte, n int) int {
	cnt := [26]int{}
	maxFreq := 0
	for _, t := range tasks {
		cnt[t-'A']++
		if cnt[t-'A'] > maxFreq {
			maxFreq = cnt[t-'A']
		}
	}
	maxCount := 0
	for _, c := range cnt {
		if c == maxFreq {
			maxCount++
		}
	}
	frame := (maxFreq-1)*(n+1) + maxCount
	if frame < len(tasks) {
		return len(tasks)
	}
	return frame
}
```

## 为什么这样写

假设最高频任务出现 `maxFreq` 次。前 `maxFreq-1` 个最高频任务后面都至少要留出一个长度为 `n+1` 的周期位置。最后一组不需要补冷却，只需要放下所有最高频任务。

如果其他任务足够多，它们能填满所有空槽，答案就是任务总数；否则答案就是骨架长度。

## 复杂度

- 时间复杂度：$O(n + 26)$。
- 空间复杂度：$O(1)$。

## 易错点

- 只考虑一个最高频任务，忽略多个任务并列最高频。
- 冷却时间为 0 时公式没有和任务总数取最大。
- 把空闲槽数量作为答案。
- 误以为必须模拟优先队列，公式更直接。

## 练习顺序

建议先刷 #621。
