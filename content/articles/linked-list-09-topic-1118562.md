---
title: 插入排序维护已排区：链表训练题解
category: 链表
summary: 链表插入排序维护一条已排序链表；每次从原链摘下当前节点，找到插入前驱，再把节点接进去。
problem_ids: [147]
order: 109
---

# 插入排序维护已排区：链表训练题解

链表插入排序和数组插入排序的思路一样：前面维护一个已排序区，每次把一个新节点插入到正确位置。链表的优势是插入不需要搬移元素，只需要改指针。

一句话记法：**先摘当前节点，再从 dummy 找插入前驱。**

## 适用场景

- 对链表做插入排序。
- 数据规模较小，或题目指定插入排序。
- 需要稳定地把节点插入到已排序链表中。

如果追求最优复杂度，链表排序通常用归并排序，时间 $O(n \log n)$。

## 图解思路

```text
sorted: dummy -> 1 -> 3 -> 5
curr:   4 -> ...

找到 pre=3，把 4 插入：
dummy -> 1 -> 3 -> 4 -> 5
```

每次插入前，必须保存 `curr.Next`，否则会丢掉未排序部分。

## 不变量

- `dummy.Next` 指向已排序链表头。
- `curr` 指向原链中下一个待插入节点。
- 插入前，已排序链表内部有序。
- 插入后，已排序链表仍然有序。

## 手写步骤

1. 建 `dummy` 作为已排序链表头前驱。
2. 遍历原链，每次保存 `next := curr.Next`。
3. 从 dummy 开始找第一个 `pre.Next.Val >= curr.Val` 的前驱。
4. 把 `curr` 插入 `pre` 后面。
5. 继续处理 `next`。

## Go 参考实现

```go
func insertionSortList(head *ListNode) *ListNode {
	dummy := &ListNode{}
	curr := head
	for curr != nil {
		next := curr.Next
		pre := dummy
		for pre.Next != nil && pre.Next.Val < curr.Val {
			pre = pre.Next
		}
		curr.Next = pre.Next
		pre.Next = curr
		curr = next
	}
	return dummy.Next
}
```

## 为什么这样写

链表没有随机访问，插入位置只能从头找。`dummy` 让插入到头部也和插入中间一样：都是插到某个 `pre` 后面。

保存 `next` 是关键，因为一旦执行 `curr.Next = pre.Next`，原链后续节点就不再能通过 `curr.Next` 找到。

## 复杂度

- 时间复杂度：最坏 $O(n^2)$。
- 空间复杂度：$O(1)$。

## 易错点

- 插入前没保存 `curr.Next`。
- 找插入位置时从 `head` 开始，而不是从已排序链表 dummy 开始。
- 比较条件用 `<=` 会改变相同值的稳定性。
- 插入后忘记 `curr = next`。

## 练习顺序

建议先刷 #147，再和 #148 归并排序对比。

插入排序练的是“摘节点再接入”，归并排序练的是“切分再合并”。
