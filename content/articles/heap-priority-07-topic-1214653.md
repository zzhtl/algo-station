---
title: 重组字符串错峰排布：堆与优先队列训练题解
category: 堆与优先队列
summary: 用剩余次数大根堆安排相邻不同字符，重点训练“上一轮字符暂缓回堆”的冷却思路。
problem_ids: [767, 621]
order: 107
---

# 重组字符串错峰排布：堆与优先队列训练题解

重组字符串要求相邻字符不能相同。最自然的贪心是每次尽量放剩余次数最多的字符，但刚放过的字符不能立刻再放，否则会造成相邻重复。

所以堆里保存“当前可以使用的字符”，而刚使用过且还有剩余次数的字符，要暂缓到下一轮再放回堆。

## 思路拆解

先做一个必要性判断：如果某个字符出现次数超过 `(n+1)/2`，一定无法重组。因为最多的字符需要被其他字符隔开，空位不够时必然相邻。

构造时维护：

- 大根堆：按剩余次数排序，取当前最需要安排的字符。
- `prev`：上一轮用过且还剩次数的字符，暂时不能参与本轮竞争。

每轮流程：

1. 从堆里弹出剩余次数最多的字符 `cur`。
2. 把 `cur` 放入答案，次数减一。
3. 上一轮暂缓的 `prev` 此时可以回堆。
4. 如果 `cur` 还有剩余，把它设为新的 `prev`。

## Go 参考实现

```go
package main

import "container/heap"

type Item struct {
	ch    byte
	count int
}

type MaxHeap []Item

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i].count > h[j].count }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(Item))
}

func (h *MaxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func reorganizeString(s string) string {
	counts := [26]int{}
	maxCount := 0
	for i := 0; i < len(s); i++ {
		idx := s[i] - 'a'
		counts[idx]++
		if counts[idx] > maxCount {
			maxCount = counts[idx]
		}
	}
	if maxCount > (len(s)+1)/2 {
		return ""
	}

	h := &MaxHeap{}
	heap.Init(h)
	for i, c := range counts {
		if c > 0 {
			heap.Push(h, Item{ch: byte('a' + i), count: c})
		}
	}

	ans := make([]byte, 0, len(s))
	var prev *Item
	for h.Len() > 0 {
		cur := heap.Pop(h).(Item)
		ans = append(ans, cur.ch)
		cur.count--

		if prev != nil {
			heap.Push(h, *prev)
			prev = nil
		}
		if cur.count > 0 {
			prev = &cur
		}
	}

	return string(ans)
}
```

## 为什么要暂缓上一轮字符

如果把 `cur` 减一后立刻放回堆，它可能仍然是最大值，下一轮又被弹出，直接造成相邻重复。暂缓一轮相当于给它设置冷却时间 `1`。

这也是任务调度题的通用思路：堆负责选择最高优先级任务，冷却队列负责控制什么时候可以重新入堆。

## 易错点

- 必须先判断最大频次，否则最后可能构造到一半无字符可选。
- `prev` 回堆要发生在本轮选完之后，不能选之前回堆。
- 这份写法默认小写字母；如果字符集更大，用 `map[byte]int` 或 `map[rune]int`。

## 复杂度

设字符种类数为 `m`，字符串长度为 `n`。每个字符被弹出和回堆的次数与出现次数相关，总时间 `O(n log m)`，空间 `O(m)`。
