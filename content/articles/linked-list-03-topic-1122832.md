---
title: 原地翻转一段链表：链表训练题解
category: 链表
summary: 区间翻转的核心不是整链反转，而是固定区间前驱 `pre`，不断把 `curr.Next` 摘出来插到 `pre` 后面。
problem_ids: [92, 206]
order: 103
---

# 原地翻转一段链表：链表训练题解

反转整条链表只需要 `prev/curr/next`。但反转一段区间时，还要把反转后的区间接回原链表。最稳的写法是头插法。

一句话记法：**`pre` 不动，`curr` 不动，每轮把 `curr.Next` 摘到 `pre` 后面。**

## 适用场景

- 反转第 `left` 到第 `right` 个节点。
- K 个一组反转的内部子操作。
- 两两交换节点也可以看成长度为 2 的区间重排。

只要操作可能从头节点开始，先加 dummy。

## 图解思路

```text
pre -> curr -> x -> y -> next

把 x 摘到 pre 后：
pre -> x -> curr -> y -> next

再把 y 摘到 pre 后：
pre -> y -> x -> curr -> next
```

`pre` 是区间前驱，`curr` 最终会变成反转区间的尾节点。

## 不变量

- `pre.Next` 始终是已反转部分的新头。
- `curr` 是已反转部分的尾。
- `curr.Next` 是下一次要摘出来头插的节点。
- 未处理部分始终挂在 `curr.Next` 后面，没有丢链。

## 手写步骤

1. 建 dummy。
2. 让 `pre` 走到 `left - 1` 位置。
3. `curr := pre.Next`。
4. 重复 `right - left` 次：摘出 `curr.Next`，插到 `pre.Next` 前。
5. 返回 `dummy.Next`。

## Go 参考实现

```go
func reverseBetween(head *ListNode, left int, right int) *ListNode {
	dummy := &ListNode{Next: head}
	pre := dummy
	for i := 1; i < left; i++ {
		pre = pre.Next
	}

	curr := pre.Next
	for i := 0; i < right-left; i++ {
		next := curr.Next
		curr.Next = next.Next
		next.Next = pre.Next
		pre.Next = next
	}
	return dummy.Next
}
```

## 为什么这样写

头插法的好处是区间两端连接关系一直稳定：

- 区间前面的节点始终是 `pre`。
- 区间后面的节点始终通过 `curr.Next` 保留。
- 每轮只移动 `curr` 后面的一个节点到前面。

这比先断开区间、整段反转、再接回去少几个临时变量，也更适合手写。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(1)$。

## 易错点

- 循环次数写成 `right-left+1`，多反转一次。
- 没保存 `next := curr.Next` 就改边。
- 反转从头开始时没有 dummy，头节点接回错误。
- 把 `curr` 每轮向后移动，破坏头插法不变量。

## 练习顺序

建议按这个顺序刷：#206, #92, #24, #25。

先写整链反转，再写区间头插；K 组反转只是多了窗口边界检查。
