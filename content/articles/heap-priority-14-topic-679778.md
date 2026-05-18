---
title: 最小化合并成本：堆与优先队列训练题解
category: 堆与优先队列
summary: 用小根堆每次合并两个最小代价，理解哈夫曼式贪心为什么能得到最小总成本。
problem_ids: [1167]
order: 114
---

# 最小化合并成本：堆与优先队列训练题解

合并石子、连接木棍这类题的规则通常是：每次合并两段，成本是两段长度之和，新段再参与后续合并。目标是总成本最小。

最优策略是每次合并当前最小的两段。这就是哈夫曼编码背后的贪心结构。

## 直觉

一段长度如果很早被合并，它会在后续合并中被重复计入多次。为了降低总成本，应该让小长度承担更多重复计入的次数，让大长度尽量晚参与。

所以每轮取最小的两个合并：

1. 从小根堆弹出 `a` 和 `b`。
2. 本轮成本增加 `a + b`。
3. 把新段 `a + b` 放回堆。
4. 堆里只剩一段时结束。

## Go 参考实现

```go
package main

import "container/heap"

type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *MinHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func connectSticks(sticks []int) int {
	if len(sticks) <= 1 {
		return 0
	}

	h := MinHeap(sticks)
	heap.Init(&h)
	cost := 0

	for h.Len() > 1 {
		a := heap.Pop(&h).(int)
		b := heap.Pop(&h).(int)
		merged := a + b
		cost += merged
		heap.Push(&h, merged)
	}

	return cost
}
```

## 贪心为什么成立

可以从交换角度理解：最终合并过程形成一棵二叉树，叶子是原始长度。某个长度对总成本的贡献等于 `长度 * 它在树中的深度`。

深度越大，说明它越早参与合并、被重复计费越多。为了最小化总成本，较小的长度应该放在更深的位置，较大的长度应该放在更浅的位置。每次把两个最小值合成一个新节点，正是在构造这种最优树。

## 易错点

- 每轮成本加的是 `a+b`，不是最终合并后的总长度。
- 合并后的新段必须回堆，因为它还会参与后续合并。
- 只有 0 或 1 段时无需合并，成本是 0。

## 复杂度

建堆 `O(n)`，合并 `n-1` 轮，每轮两次弹出一次插入，时间 `O(n log n)`，空间 `O(n)`。
