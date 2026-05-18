---
title: 按收益解锁项目：堆与优先队列训练题解
category: 堆与优先队列
summary: 用资本排序加收益大根堆拆解 IPO 类题，训练“先解锁候选，再选择最优”的贪心结构。
problem_ids: [502]
order: 106
---

# 按收益解锁项目：堆与优先队列训练题解

IPO 类题有两个条件：项目有启动资本 `capital[i]`，完成后获得利润 `profits[i]`，最多做 `k` 个项目，目标是最终资本最大。

这题不是简单按利润排序，因为高利润项目可能暂时做不了；也不是每轮扫描所有项目，因为会退化到 `O(k n)`。正确拆法是：资本决定能不能进入候选池，利润决定候选池里选谁。

## 两层结构

先把项目按启动资本从小到大排序，维护指针 `idx`：

- 当前资本是 `w`。
- 所有 `capital <= w` 的项目都已经解锁，放进收益大根堆。
- 每轮从收益大根堆里取利润最高的项目做掉，更新 `w`。
- 新资本可能继续解锁更多项目。

不变量：

- 排序数组中 `idx` 之前的项目都已经进入过候选池。
- 堆中保存“已解锁但还没做”的项目利润。
- 每轮选择堆顶利润最大项目是局部最优，也不会影响已经解锁集合之外的项目顺序。

## Go 参考实现

```go
package main

import (
	"container/heap"
	"sort"
)

type Project struct {
	capital int
	profit  int
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

func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	projects := make([]Project, len(profits))
	for i := range profits {
		projects[i] = Project{capital: capital[i], profit: profits[i]}
	}
	sort.Slice(projects, func(i, j int) bool {
		return projects[i].capital < projects[j].capital
	})

	h := &MaxHeap{}
	heap.Init(h)
	idx := 0

	for step := 0; step < k; step++ {
		for idx < len(projects) && projects[idx].capital <= w {
			heap.Push(h, projects[idx].profit)
			idx++
		}
		if h.Len() == 0 {
			break
		}
		w += heap.Pop(h).(int)
	}

	return w
}
```

## 贪心为什么成立

在某一轮，所有可做项目已经完整进入堆。做利润更高的项目只会让下一轮资本不小于做低利润项目后的资本，因此它不会减少未来可解锁项目集合，反而可能更早解锁项目。

所以每轮在“当前已解锁集合”里选最大利润是安全的。排序负责扩大候选集合，堆负责从候选集合中选最优。

## 易错点

- 不能一开始把所有项目按利润入堆，否则会选到资本不够的项目。
- 每轮选项目前都要把新解锁项目全部推入堆，而不是只推一个。
- 堆空时说明当前资本无法解锁任何剩余项目，应提前结束。

## 复杂度

排序 `O(n log n)`；每个项目最多入堆一次，最多做 `k` 次弹出，总时间 `O(n log n + k log n)`，空间 `O(n)`。
