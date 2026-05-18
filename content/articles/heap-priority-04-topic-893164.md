---
title: 多路归并弹最小头：堆与优先队列训练题解
category: 堆与优先队列
summary: 把 K 条有序链表或数组看成 K 路候选流，用小根堆每次取全局最小头并推进对应来源。
problem_ids: [23, 373, 378]
order: 104
---

# 多路归并弹最小头：堆与优先队列训练题解

多路归并的核心动作只有一个：每条有序序列当前只暴露一个头元素，所有头元素里最小的那个，一定是全局下一个输出。

堆在这里不是保存全部元素，而是保存“每一路当前的头”。弹出一个头以后，再把同一路的下一个元素补进堆。

## 状态设计

以合并 K 个升序链表为例，堆节点需要记录：

- `val`：当前节点值，用来比较大小。
- `list` 或节点指针：知道弹出后从哪一路继续推进。

不变量：

- 堆中每一路最多有一个候选头。
- 堆顶是所有未合并节点中的最小值。
- 弹出堆顶后，只推进它所在的那一路。

## Go 参考实现

```go
package main

import "container/heap"

type ListNode struct {
	Val  int
	Next *ListNode
}

type NodeHeap []*ListNode

func (h NodeHeap) Len() int           { return len(h) }
func (h NodeHeap) Less(i, j int) bool { return h[i].Val < h[j].Val }
func (h NodeHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *NodeHeap) Push(x any) {
	*h = append(*h, x.(*ListNode))
}

func (h *NodeHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func mergeKLists(lists []*ListNode) *ListNode {
	h := &NodeHeap{}
	heap.Init(h)

	for _, node := range lists {
		if node != nil {
			heap.Push(h, node)
		}
	}

	dummy := &ListNode{}
	tail := dummy
	for h.Len() > 0 {
		node := heap.Pop(h).(*ListNode)
		tail.Next = node
		tail = tail.Next
		if node.Next != nil {
			heap.Push(h, node.Next)
		}
	}

	return dummy.Next
}
```

## 为什么只放每一路的头

因为每一路内部已经有序。如果某一路当前头都不是最小，那它后面的元素更不可能先输出；如果某一路当前头被弹出，只有它的下一个元素才获得竞争资格。

这和 BFS 的“扩展当前节点”很像，只是扩展规则被有序性限制成了“同一路向后一步”。

## 扩展到数组数对

LeetCode 373 的 K 个最小数对也是多路归并：固定 `nums1[i]`，随着 `j` 增大，`nums1[i]+nums2[j]` 单调不降。初始把每个 `i` 的 `(i,0)` 入堆；弹出 `(i,j)` 后再推 `(i,j+1)`。

这种题要警惕重复入堆。若扩展方向不止一个，比如矩阵里同时向右和向下扩展，通常需要 `visited` 去重。

## 易错点

- 链表题里不要忘了空链表过滤。
- 堆节点必须携带来源信息，否则弹出后不知道推进哪一路。
- 合并链表时复用原节点即可；如果题目要求不修改原结构，再新建节点。

## 复杂度

设总节点数为 `N`，链表条数为 `k`。堆大小最多 `k`，总时间 `O(N log k)`，空间 `O(k)`。
