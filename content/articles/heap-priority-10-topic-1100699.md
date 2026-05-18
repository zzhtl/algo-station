---
title: 滑动窗口堆顶校验：堆与优先队列训练题解
category: 堆与优先队列
summary: 把窗口边界写进堆顶有效性判断，训练先清理再读取答案的顺序。
problem_ids: [239]
order: 110
---

# 滑动窗口堆顶校验：堆与优先队列训练题解

滑动窗口里用堆，真正要练的是“堆顶校验”。堆内部可能有过期元素，但答案只从堆顶读取，因此只要保证读取前堆顶有效即可。

以窗口最大值为例，堆顶校验条件是：

```text
堆顶下标 >= 当前窗口左边界
```

不满足就弹掉，直到堆顶属于窗口。

## 固定写法

对每个右端点 `right`：

1. 新元素 `(nums[right], right)` 入堆。
2. 计算左边界 `left = right - k + 1`。
3. 循环弹出 `idx < left` 的堆顶。
4. 当 `left >= 0` 时，记录堆顶值。

注意第 3 步必须是 `for`，不是 `if`。因为堆顶可能连续多个都已经过期。

## Go 参考实现

```go
package main

import "container/heap"

type Entry struct {
	value int
	index int
}

type MaxHeap []Entry

func (h MaxHeap) Len() int { return len(h) }
func (h MaxHeap) Less(i, j int) bool {
	if h[i].value == h[j].value {
		return h[i].index > h[j].index
	}
	return h[i].value > h[j].value
}
func (h MaxHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(Entry))
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

	for right, value := range nums {
		heap.Push(h, Entry{value: value, index: right})
		left := right - k + 1

		for h.Len() > 0 && (*h)[0].index < left {
			heap.Pop(h)
		}
		if left >= 0 {
			ans = append(ans, (*h)[0].value)
		}
	}

	return ans
}
```

## 和单调队列的取舍

滑动窗口最大值最优解是单调队列，时间 `O(n)`。堆写法是 `O(n log n)`，但它有迁移价值：当优先级不是单纯的值大小，或者窗口状态还和其他排序条件绑定时，堆加校验更通用。

训练时可以先用堆理解“过期校验”，再用单调队列优化掉被压在堆里的无效候选。

## 易错点

- 不能在窗口形成前记录答案；`left >= 0` 才说明窗口长度达到 `k`。
- 过期条件是 `< left`，不是 `<= left`；下标等于左边界仍然在窗口内。
- 堆顶值相等时可用下标做 tie-breaker，让较新的元素优先，减少过期堆顶出现次数。

## 复杂度

每个元素入堆一次、过期后最多弹出一次，总时间 `O(n log n)`，空间 `O(n)`。
