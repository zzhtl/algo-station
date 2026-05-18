---
title: 双堆维护数据流中位数：堆与优先队列训练题解
category: 堆与优先队列
summary: 用左大根堆和右小根堆拆开有序序列，重点训练平衡大小、跨堆顺序和中位数读取。
problem_ids: [295]
order: 103
---

# 双堆维护数据流中位数：堆与优先队列训练题解

数据流中位数难在“每次插入后都要查询”。如果每次排序，插入成本太高；如果只维护一个堆，又拿不到中间位置。双堆的想法是把有序序列从中间切开。

## 两个堆的含义

- `small`：大根堆，保存较小的一半，堆顶是左半部分最大值。
- `large`：小根堆，保存较大的一半，堆顶是右半部分最小值。

始终维护两个不变量：

- 大小平衡：`len(small) == len(large)` 或 `len(small) == len(large)+1`。
- 顺序正确：`small` 里的每个数都不大于 `large` 里的每个数。

这样一来：

- 总数为奇数，中位数是 `small` 堆顶。
- 总数为偶数，中位数是两个堆顶的平均值。

## Go 参考实现

```go
package main

import "container/heap"

type Heap struct {
	data []int
	less func(a, b int) bool
}

func (h Heap) Len() int           { return len(h.data) }
func (h Heap) Less(i, j int) bool { return h.less(h.data[i], h.data[j]) }
func (h Heap) Swap(i, j int)      { h.data[i], h.data[j] = h.data[j], h.data[i] }

func (h *Heap) Push(x any) {
	h.data = append(h.data, x.(int))
}

func (h *Heap) Pop() any {
	old := h.data
	x := old[len(old)-1]
	h.data = old[:len(old)-1]
	return x
}

func (h *Heap) Top() int {
	return h.data[0]
}

type MedianFinder struct {
	small *Heap
	large *Heap
}

func Constructor() MedianFinder {
	small := &Heap{less: func(a, b int) bool { return a > b }}
	large := &Heap{less: func(a, b int) bool { return a < b }}
	heap.Init(small)
	heap.Init(large)
	return MedianFinder{small: small, large: large}
}

func (m *MedianFinder) AddNum(num int) {
	if m.small.Len() == 0 || num <= m.small.Top() {
		heap.Push(m.small, num)
	} else {
		heap.Push(m.large, num)
	}

	if m.small.Len() > m.large.Len()+1 {
		heap.Push(m.large, heap.Pop(m.small).(int))
	} else if m.large.Len() > m.small.Len() {
		heap.Push(m.small, heap.Pop(m.large).(int))
	}
}

func (m *MedianFinder) FindMedian() float64 {
	if m.small.Len() > m.large.Len() {
		return float64(m.small.Top())
	}
	return float64(m.small.Top()+m.large.Top()) / 2.0
}
```

## 为什么插入后要再平衡

插入时只能根据 `small.Top()` 粗略判断放左边还是右边，这一步保证顺序大体正确；但插入可能让某一边数量超标，所以必须再做一次搬运。

搬运只从堆顶搬：

- `small` 太多，就把左半部分最大值搬到右边。
- `large` 太多，就把右半部分最小值搬到左边。

这样搬运不会破坏跨堆顺序，因为被搬的正好是边界值。

## 易错点

- 不能只保证大小平衡，不保证顺序；否则堆顶平均值可能不是中位数。
- 偶数个元素求平均时注意转成 `float64`，更稳妥的写法是 `float64(a)/2 + float64(b)/2`，避免极大整数相加溢出。
- 约定 `small` 多一个元素后，奇数场景读取逻辑更简单。

## 复杂度

每次插入最多触发一次入堆和一次跨堆搬运，时间 `O(log n)`；查询中位数 `O(1)`；空间 `O(n)`。
