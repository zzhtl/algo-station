---
title: K 组翻转的边界检查：链表训练题解
category: 链表
summary: K 个一组反转的关键不是反转本身，而是每轮先确认后面够 K 个节点；不够时尾部必须保持原样。
problem_ids: [25, 24]
order: 108
---

# K 组翻转的边界检查：链表训练题解

K 个一组翻转是区间翻转的循环版。真正容易错的是边界：每轮必须先确认后面还有 K 个节点，不足 K 个不能动。

一句话记法：**先找本组 tail；找不到就结束；找到了再反转。**

## 适用场景

- 每 K 个节点为一组反转。
- 两两交换节点，可以看成 `k = 2`。
- 需要保持不足 K 个的尾部原样。

这类题建议先写一个能反转 `[head, tail]` 或长度 K 窗口的局部操作。

## 图解思路

```text
pre -> head -> ... -> tail -> nextGroup

反转本组后：
pre -> tail -> ... -> head -> nextGroup

下一轮 pre = head
```

反转后，原来的 `head` 会变成本组尾部，也就是下一轮的前驱。

## 不变量

- `pre` 是当前待反转组的前一个节点。
- `tail` 是当前组第 K 个节点。
- `nextGroup` 是下一组头节点。
- 反转完成后，`pre` 移动到本组尾部，也就是旧 `head`。

## 手写步骤

1. 建 dummy。
2. 令 `pre := dummy`。
3. 从 `pre` 出发向后找 K 步得到 `tail`。
4. 如果不够 K 个，返回。
5. 记录 `head := pre.Next` 和 `nextGroup := tail.Next`。
6. 断开、反转本组、接回。
7. `pre = head`，进入下一组。

## Go 参考实现

```go
func reverseKGroup(head *ListNode, k int) *ListNode {
	dummy := &ListNode{Next: head}
	pre := dummy

	for {
		tail := pre
		for i := 0; i < k && tail != nil; i++ {
			tail = tail.Next
		}
		if tail == nil {
			break
		}

		groupHead := pre.Next
		nextGroup := tail.Next
		tail.Next = nil
		pre.Next = reverseList(groupHead)
		groupHead.Next = nextGroup
		pre = groupHead
	}
	return dummy.Next
}

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	curr := head
	for curr != nil {
		next := curr.Next
		curr.Next = prev
		prev = curr
		curr = next
	}
	return prev
}
```

## 为什么这样写

必须先找 `tail`，因为题目要求不足 K 个不反转。如果边走边反转，走到尾部才发现不够，就还要恢复，复杂且容易错。

断开 `tail.Next = nil` 后，可以复用普通整链反转函数。反转后旧 `groupHead` 变成本组尾部，把它接到 `nextGroup`，再让 `pre` 移到旧 `groupHead`。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(1)$，迭代反转。

## 易错点

- 没检查够不够 K 个就反转。
- 反转后忘记接回 `nextGroup`。
- 下一轮 `pre` 没移动到旧头节点。
- `tail.Next = nil` 后忘记提前保存 `nextGroup`。

## 练习顺序

建议按这个顺序刷：#24, #25。

先做两两交换，理解一组反转后的前驱移动；再推广到任意 K。
