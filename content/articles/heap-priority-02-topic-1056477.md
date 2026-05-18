---
title: 大根堆弹出当前最大：堆与优先队列训练题解
category: 堆与优先队列
summary: 从“每次取当前最大”的角度理解大根堆，训练 Go 里用负号或反向 Less 表达最大优先级。
problem_ids: [1046, 2530]
order: 102
---

# 大根堆弹出当前最大：堆与优先队列训练题解

大根堆适合“每一步都必须拿当前最大值做决策”的题。它的重点不是排序，而是允许你在修改一个元素后，快速重新得到新的最大值。

典型题是“最后一块石头重量”：每轮取出最大的两块，碰撞后如果还有剩余，把剩余重量放回去。这里最大值会被反复删除和插入，排序数组每次维护成本太高，堆正好匹配。

## 决策过程

维护一个大根堆 `heap`：

- 堆顶永远是当前最大重量。
- 每轮弹出两个最大值 `a >= b`。
- 如果 `a != b`，把 `a - b` 放回堆。
- 堆中少于两个元素时结束。

这个过程的正确性来自题意本身：每轮指定拿最重的两块，堆只是把“找最重”从线性扫描降为对数复杂度。

## Go 参考实现

```go
package main

import "container/heap"

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

func lastStoneWeight(stones []int) int {
	h := MaxHeap(stones)
	heap.Init(&h)

	for h.Len() > 1 {
		a := heap.Pop(&h).(int)
		b := heap.Pop(&h).(int)
		if a != b {
			heap.Push(&h, a-b)
		}
	}

	if h.Len() == 0 {
		return 0
	}
	return h[0]
}
```

## 和小根堆 Top K 的区别

Top K 是“保留固定数量的候选”，常用小根堆淘汰最弱者；大根堆弹最大是“每轮消费最强候选”，堆的大小会随着处理过程变化。

判断该用哪种堆，可以看堆顶代表什么：

- 堆顶是答案候选里的淘汰门槛：多半是固定容量小根堆。
- 堆顶是下一步必须处理的对象：多半是大根堆或按优先级排序的堆。

## 易错点

- Go 里写大根堆，只需要把 `Less` 改成 `>`；不要在同一份代码里又取负数又反向 `Less`。
- 弹两个元素之前必须保证 `Len() > 1`。
- 如果两个最大值相等，本轮没有元素放回；不要把 `0` 再入堆。

## 复杂度

建堆 `O(n)`，每轮最多两次弹出一次插入，单轮 `O(log n)`，总时间 `O(n log n)`，空间 `O(n)`。

## 练习顺序

先用 1046 练习大根堆的基本弹出和回填；再做 2530 这类“取最大、修改、放回、累加答案”的题，体会堆维护动态优先级的价值。
