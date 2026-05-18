---
title: 距离原点最近 K 点：堆与优先队列训练题解
category: 堆与优先队列
summary: 用固定容量大根堆维护 K 个最小距离，重点训练“堆顶是当前最差候选”的反向不变量。
problem_ids: [973]
order: 108
---

# 距离原点最近 K 点：堆与优先队列训练题解

求最近 K 点，本质是 Top K 小。和第 K 大相反，这里要保留 K 个最小距离，所以堆顶应该放“当前 K 个候选里最远的点”。新点如果比堆顶更近，就替换堆顶。

## 不变量

维护一个大小最多为 `k` 的大根堆，堆节点保存：

- `dist`：点到原点的距离平方。
- `point`：原始坐标。

堆满足：

- 堆里是目前见过的 K 个最近点。
- 堆顶是这 K 个点里最远的一个。
- 新点距离不小于堆顶时，它不可能进入答案。

距离不用开方，比较 `x*x + y*y` 即可。

## Go 参考实现

```go
package main

import "container/heap"

type Point struct {
	dist int
	x    int
	y    int
}

type MaxHeap []Point

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i].dist > h[j].dist }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(Point))
}

func (h *MaxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func kClosest(points [][]int, k int) [][]int {
	h := &MaxHeap{}
	heap.Init(h)

	for _, p := range points {
		d := p[0]*p[0] + p[1]*p[1]
		item := Point{dist: d, x: p[0], y: p[1]}
		if h.Len() < k {
			heap.Push(h, item)
			continue
		}
		if d < (*h)[0].dist {
			heap.Pop(h)
			heap.Push(h, item)
		}
	}

	ans := make([][]int, 0, k)
	for h.Len() > 0 {
		p := heap.Pop(h).(Point)
		ans = append(ans, []int{p.x, p.y})
	}
	return ans
}
```

## 为什么用大根堆

保留 K 个最小值时，最需要被淘汰的是候选中最大的那个。大根堆把“最差候选”放在堆顶，替换逻辑就变成一次比较。

这个思路可以迁移到很多“Top K 小”题：

- K 个最低成本。
- K 个最近元素。
- K 个最小数对。

如果题目要求结果有序，最后再排序答案；如果只要求任意顺序，堆弹出的顺序不重要。

## 易错点

- 不需要计算欧氏距离的平方根，平方距离的大小关系相同。
- 如果坐标范围很大，`x*x + y*y` 可能需要 `int64`。
- 固定容量堆的空间是 `O(k)`；把所有点入堆再弹 K 次会变成 `O(n)` 空间。

## 复杂度

遍历 `n` 个点，每次堆操作 `O(log k)`，总时间 `O(n log k)`，空间 `O(k)`。
