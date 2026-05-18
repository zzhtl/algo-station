---
title: 堆加懒删除处理过期：堆与优先队列训练题解
category: 堆与优先队列
summary: 用“入堆时带位置、取堆顶时校验”的方式处理滑动窗口和动态删除，重点掌握懒删除边界。
problem_ids: [239, 480]
order: 105
---

# 堆加懒删除处理过期：堆与优先队列训练题解

普通堆只擅长删除堆顶，不擅长删除任意位置的元素。滑动窗口里元素离开窗口时，它可能埋在堆内部，强行删除会破坏堆的简洁性。

懒删除的做法是：离开的元素先不管，等它将来浮到堆顶时，再判断它是否过期，过期就弹掉。

## 以滑动窗口最大值为例

堆里放 `(value, index)`，用大根堆按 `value` 排序。

处理到右端点 `i` 时：

1. 把 `(nums[i], i)` 入堆。
2. 窗口左边界是 `left = i-k+1`。
3. 在读取答案前，不断弹出 `index < left` 的堆顶。
4. 堆顶就是当前窗口最大值。

不变量不是“堆里全是窗口内元素”，而是更弱也更容易维护的：“清理堆顶后，堆顶一定属于当前窗口”。

## Go 参考实现

```go
package main

import "container/heap"

type Pair struct {
	val int
	idx int
}

type MaxHeap []Pair

func (h MaxHeap) Len() int { return len(h) }
func (h MaxHeap) Less(i, j int) bool {
	if h[i].val == h[j].val {
		return h[i].idx > h[j].idx
	}
	return h[i].val > h[j].val
}
func (h MaxHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(Pair))
}

func (h *MaxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func maxSlidingWindow(nums []int, k int) []int {
	h := &MaxHeap{}
	heap.Init(h)
	ans := make([]int, 0, len(nums)-k+1)

	for i, x := range nums {
		heap.Push(h, Pair{val: x, idx: i})
		left := i - k + 1
		for h.Len() > 0 && (*h)[0].idx < left {
			heap.Pop(h)
		}
		if left >= 0 {
			ans = append(ans, (*h)[0].val)
		}
	}

	return ans
}
```

## 懒删除的适用边界

适合用懒删除的条件：

- 删除对象可以通过额外字段判断是否有效，比如下标、版本号、计数。
- 最终只关心堆顶是否有效，不要求堆内部完全干净。
- 每个过期元素最多被弹出一次，摊还复杂度可控。

如果题目频繁需要删除任意元素且马上影响非堆顶查询，堆就不够了，可能要用平衡树、双端队列或哈希表加链表。

## 易错点

- 必须在读取答案前清理堆顶；入堆后立刻读答案会读到过期元素。
- 只用值判断过期不可靠，重复值会混淆；滑动窗口题应带下标。
- 堆里可能残留很多过期元素，这是设计的一部分，不是 bug。

## 复杂度

每个元素入堆一次、最多弹出一次，总时间 `O(n log n)`，空间 `O(n)`。滑动窗口最大值的最优解是单调队列 `O(n)`，但堆加懒删除能迁移到更复杂的优先级场景。
