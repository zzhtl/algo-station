---
title: 小根堆维护 Top K 大：堆与优先队列训练题解
category: 堆与优先队列
summary: 用小根堆把“第 K 大”和“前 K 大”转成固定容量候选集，重点训练堆顶不变量和淘汰逻辑。
problem_ids: [215, 703]
order: 101
---

# 小根堆维护 Top K 大：堆与优先队列训练题解

Top K 大的关键不是把所有数排好，而是维护一个大小最多为 `k` 的候选集：候选集里放目前见过的 K 个最大数，堆顶就是这 K 个数里最小的那个，也就是当前的“第 K 大门槛”。

## 不变量

小根堆 `heap` 始终满足：

- `len(heap) <= k`。
- 当已经处理过至少 `k` 个数时，`heap` 中正好是已处理元素里的 K 个最大值。
- `heap[0]` 是这 K 个最大值里最小的，所以它就是当前第 K 大。

新数 `x` 来了以后，只问一个问题：`x` 有没有资格挤进前 K 大？

- 堆还没满：直接入堆。
- 堆已满且 `x <= heap[0]`：它连门槛都没超过，丢掉。
- 堆已满且 `x > heap[0]`：弹出门槛，把 `x` 放进去。

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

func findKthLargest(nums []int, k int) int {
	h := &MinHeap{}
	heap.Init(h)

	for _, x := range nums {
		if h.Len() < k {
			heap.Push(h, x)
			continue
		}
		if x > (*h)[0] {
			heap.Pop(h)
			heap.Push(h, x)
		}
	}

	return (*h)[0]
}
```

## 为什么这样写

排序会把所有相对顺序都算出来，但第 K 大只需要知道“谁在前 K 个候选里”。小根堆把最弱的候选放在堆顶，任何比它还小的数都不可能进入答案；任何比它大的数都能替换它。这就是固定容量堆的核心。

这类题不要先想“大根堆弹 K 次”。大根堆当然能做，但需要把所有元素都入堆，空间是 `O(n)`；小根堆只保留 K 个候选，空间是 `O(k)`，更适合数据流和在线更新。

## 易错点

- 比较条件写成 `>=` 不会影响第 K 大的数值，但会引入无意义替换；写成 `>` 更干净。
- `k == 1` 时堆顶就是最大值；`k == len(nums)` 时堆顶就是最小值，代码不需要特殊分支。
- Go 的 `container/heap` 默认靠 `Less` 决定堆序，小根堆就是 `h[i] < h[j]`。

## 复杂度

每个元素最多触发一次入堆或替换，单次 `O(log k)`，总时间 `O(n log k)`，空间 `O(k)`。

## 练习顺序

先做 LeetCode 215，确认固定容量小根堆；再做 703，把同一个不变量改成数据流类的 `Add` 操作。
