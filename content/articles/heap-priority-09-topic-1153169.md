---
title: 雇佣工人按比率排序：堆与优先队列训练题解
category: 堆与优先队列
summary: 通过工资质量比固定统一单价，再用大根堆保留质量和最小的 K 人候选。
problem_ids: [857]
order: 109
---

# 雇佣工人按比率排序：堆与优先队列训练题解

雇佣 K 名工人时，每个人有 `quality` 和最低工资 `wage`。如果一组工人使用同一个单位质量价格 `r` 支付，那么第 `i` 个工人的工资是 `quality[i] * r`，并且必须满足 `quality[i] * r >= wage[i]`。

所以每个工人都有最低可接受比率 `wage[i] / quality[i]`。一组人的统一比率必须不小于组内所有人的最大比率。

## 核心转化

把工人按比率从小到大排序。当遍历到某个工人时，假设他是当前组里比率最高的人，那么统一比率就固定为 `r`。此时成本变成：

```text
成本 = r * 组内 quality 总和
```

在比率已固定的前提下，要让成本最小，就要从已经遍历过的工人里选 K 个质量和最小的人。

这正好用大根堆维护 K 个最小 `quality`：

- 堆里放当前选中的质量。
- 如果超过 K 人，就弹出最大质量，降低质量总和。
- 当堆大小为 K 时，用当前比率更新答案。

## Go 参考实现

```go
package main

import (
	"container/heap"
	"math"
	"sort"
)

type Worker struct {
	quality int
	ratio   float64
}

type MaxHeap []int

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *MaxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func mincostToHireWorkers(quality []int, wage []int, k int) float64 {
	workers := make([]Worker, len(quality))
	for i := range quality {
		workers[i] = Worker{
			quality: quality[i],
			ratio:   float64(wage[i]) / float64(quality[i]),
		}
	}
	sort.Slice(workers, func(i, j int) bool {
		return workers[i].ratio < workers[j].ratio
	})

	h := &MaxHeap{}
	heap.Init(h)
	sumQuality := 0
	ans := math.MaxFloat64

	for _, w := range workers {
		heap.Push(h, w.quality)
		sumQuality += w.quality
		if h.Len() > k {
			sumQuality -= heap.Pop(h).(int)
		}
		if h.Len() == k {
			cost := float64(sumQuality) * w.ratio
			if cost < ans {
				ans = cost
			}
		}
	}

	return ans
}
```

## 为什么排序比率是关键

如果不固定“谁是最大比率”，就无法比较不同组合，因为每个组合的统一单价都可能不同。排序后遍历到 `w` 时，前面所有工人的比率都不高于 `w.ratio`，他们都能在这个单价下被合法支付。

于是问题从“同时选人和定价”降维成“在合法候选里选 K 个质量和最小的人”。

## 易错点

- 堆里维护的是 `quality`，不是工资；比率固定后成本才和质量和成正比。
- 必须用大根堆，因为超过 K 人时要删掉质量最大的那个。
- 浮点答案通常允许误差，不要把比率提前截断成整数。

## 复杂度

排序 `O(n log n)`，每个工人入堆一次、最多出堆一次，总时间 `O(n log n)`，空间 `O(n)` 或堆空间 `O(k)`。
