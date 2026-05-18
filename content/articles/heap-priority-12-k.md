---
title: 第 K 小数对枚举：堆与优先队列训练题解
category: 堆与优先队列
summary: 把每个 nums1[i] 对应的一行数对看成有序流，用小根堆做 K 次多路归并。
problem_ids: [373]
order: 112
---

# 第 K 小数对枚举：堆与优先队列训练题解

给两个升序数组，求和最小的 K 个数对。不要枚举所有 `n*m` 个数对，关键是利用有序性：固定 `nums1[i]` 后，`nums1[i] + nums2[j]` 随着 `j` 增大而增大。

所以每个 `i` 都是一条有序流：

```text
(i,0), (i,1), (i,2), ...
```

多条流合在一起，取前 K 小，就可以用小根堆做多路归并。

## 堆节点

堆里保存 `(sum, i, j)`：

- `sum` 用来比较大小。
- `i, j` 用来还原数对。
- 弹出 `(i,j)` 后，只需要推进同一行的 `(i,j+1)`。

初始时，不需要把所有 `i` 都放进去，最多放 `min(len(nums1), k)` 行，因为答案只取 K 个，超过 K 行不可能都在前 K 次弹出中贡献首元素。

## Go 参考实现

```go
package main

import "container/heap"

type Pair struct {
	sum int
	i   int
	j   int
}

type MinHeap []Pair

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i].sum < h[j].sum }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(Pair))
}

func (h *MinHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func kSmallestPairs(nums1 []int, nums2 []int, k int) [][]int {
	if len(nums1) == 0 || len(nums2) == 0 || k == 0 {
		return nil
	}

	h := &MinHeap{}
	heap.Init(h)
	limit := len(nums1)
	if k < limit {
		limit = k
	}
	for i := 0; i < limit; i++ {
		heap.Push(h, Pair{sum: nums1[i] + nums2[0], i: i, j: 0})
	}

	ans := make([][]int, 0, k)
	for h.Len() > 0 && len(ans) < k {
		cur := heap.Pop(h).(Pair)
		ans = append(ans, []int{nums1[cur.i], nums2[cur.j]})

		if cur.j+1 < len(nums2) {
			nextJ := cur.j + 1
			heap.Push(h, Pair{
				sum: nums1[cur.i] + nums2[nextJ],
				i:   cur.i,
				j:   nextJ,
			})
		}
	}

	return ans
}
```

## 为什么不会漏

每一行内部是升序的。某一行的 `(i,j+1)` 只有在 `(i,j)` 被弹出后才可能成为这一行的最小未使用候选，因此延迟入堆不会漏掉更小答案。

堆中始终保存每条活跃行的最小未使用数对，堆顶就是所有未输出数对中的最小者。弹 K 次就是前 K 小。

## 易错点

- 初始只推每行的 `j=0`，不要把所有数对入堆。
- 弹出后推进的是同一个 `i` 的 `j+1`。
- 如果两个数组可能很大，`sum` 可改成 `int64`。

## 复杂度

堆大小最多 `min(len(nums1), k)`，弹出最多 K 次，总时间 `O(k log min(n, k))`，空间 `O(min(n, k))`。
