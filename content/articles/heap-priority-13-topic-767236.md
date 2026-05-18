---
title: 实时排行榜更新：堆与优先队列训练题解
category: 堆与优先队列
summary: 用版本号或当前分数表解决堆中旧记录问题，训练动态排行榜里的懒删除模型。
problem_ids: [2353, 3092]
order: 113
---

# 实时排行榜更新：堆与优先队列训练题解

排行榜题常见操作是：更新某个对象的分数，然后查询当前最高分对象。难点是堆不能高效删除对象的旧分数，直接更新会留下过期记录。

解决办法是把每次更新都当成一条新记录入堆，同时用哈希表保存对象的当前真实分数。查询堆顶时，如果堆顶记录和哈希表不一致，就弹掉。

## 状态设计

以“食物评分系统”这类题为例：

- `score[name]`：对象当前真实分数。
- `heap`：保存历史记录 `(score, name)`。
- 查询时，堆顶只有在 `score[name] == heapTop.score` 时才有效。

如果有平分规则，比如分数高优先、名称字典序小优先，比较函数也要同时体现这两个条件。

## Go 参考实现

```go
package main

import "container/heap"

type Record struct {
	name  string
	score int
}

type RankHeap []Record

func (h RankHeap) Len() int { return len(h) }
func (h RankHeap) Less(i, j int) bool {
	if h[i].score == h[j].score {
		return h[i].name < h[j].name
	}
	return h[i].score > h[j].score
}
func (h RankHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *RankHeap) Push(x any) {
	*h = append(*h, x.(Record))
}

func (h *RankHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

type Leaderboard struct {
	score map[string]int
	h     *RankHeap
}

func NewLeaderboard() *Leaderboard {
	h := &RankHeap{}
	heap.Init(h)
	return &Leaderboard{
		score: map[string]int{},
		h:     h,
	}
}

func (b *Leaderboard) Update(name string, score int) {
	b.score[name] = score
	heap.Push(b.h, Record{name: name, score: score})
}

func (b *Leaderboard) Top() string {
	for b.h.Len() > 0 {
		top := (*(b.h))[0]
		if b.score[top.name] == top.score {
			return top.name
		}
		heap.Pop(b.h)
	}
	return ""
}
```

## 为什么不用直接修改堆中元素

理论上可以记录每个对象在堆数组里的位置，然后用 `heap.Fix` 更新。但这会让实现复杂很多：每次交换都要维护下标映射，删除也要同步映射。

懒删除更适合刷题和业务里的事件流：更新只追加记录，查询时清理旧记录。只要每条旧记录最多被弹一次，总体复杂度仍然可控。

## 易错点

- 堆顶有效性必须和当前表比较，不能只相信堆里最新入堆时间。
- 平分规则要写进 `Less`，否则答案在同分时会错。
- 如果排行榜按类别分组，每个类别要有自己的堆和当前分数表。

## 复杂度

每次更新 `O(log m)`，查询时可能弹出若干过期记录，但每条记录只会被弹一次，所以摊还 `O(log m)`。空间与更新次数相关，必要时可定期重建堆。
