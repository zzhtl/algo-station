---
title: 删除重复节点的前驱技巧：链表训练题解
category: 链表
summary: 有序链表去重分两类：保留一个重复值时移动当前节点；删除整段重复值时必须让前驱跳过整段。
problem_ids: [83, 82]
order: 114
---

# 删除重复节点的前驱技巧：链表训练题解

有序链表去重有两个常见版本：#83 保留每个值一个节点，#82 删除所有重复值，只留下从未重复过的节点。两者看起来像，指针站位完全不同。

一句话记法：**保留一个看 `curr`，删除整段看 `prev.Next`。**

## 适用场景

- 输入链表已经有序。
- 重复值一定连续出现。
- 需要保留一个重复值，或删除整段重复值。

如果链表无序，重复节点不连续，通常要借助哈希表或先排序。

## 两种题型

保留一个重复值：

```text
1 -> 1 -> 2 -> 3 -> 3
变成 1 -> 2 -> 3
```

删除所有重复值：

```text
1 -> 1 -> 2 -> 3 -> 3
变成 2
```

#82 可能删除头部整段，所以必须使用 dummy。

## 不变量

保留一个：

- `curr` 指向当前保留下来的节点。
- 如果 `curr.Val == curr.Next.Val`，跳过 `curr.Next`。
- 否则 `curr` 前进。

删除整段：

- `prev.Next` 是当前待检查段的第一个节点。
- 如果这一段有重复，`prev.Next` 直接跳到重复段之后。
- 如果没有重复，`prev` 前进到这一段节点。

## Go 参考实现：保留一个

```go
func deleteDuplicates(head *ListNode) *ListNode {
	curr := head
	for curr != nil && curr.Next != nil {
		if curr.Val == curr.Next.Val {
			curr.Next = curr.Next.Next
		} else {
			curr = curr.Next
		}
	}
	return head
}
```

## Go 参考实现：删除整段重复

```go
func deleteDuplicatesAll(head *ListNode) *ListNode {
	dummy := &ListNode{Next: head}
	prev := dummy

	for prev.Next != nil {
		curr := prev.Next
		duplicated := false
		for curr.Next != nil && curr.Val == curr.Next.Val {
			curr = curr.Next
			duplicated = true
		}
		if duplicated {
			prev.Next = curr.Next
		} else {
			prev = prev.Next
		}
	}
	return dummy.Next
}
```

## 为什么这样写

有序是去重成立的基础，因为相同值会挨在一起。#83 只要跳过后续重复节点即可，当前第一个节点要保留，所以用 `curr` 操作。

#82 要删除整段重复值。如果 `prev.Next` 到 `curr` 这一段发生重复，整段都不能保留，`prev` 不能前进，必须直接接到 `curr.Next`。如果没有重复，`prev` 才能前进。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(1)$。

## 易错点

- #82 删除重复段后还移动 `prev`，会跳过新接上来的节点。
- 没有 dummy，头部重复段删除困难。
- 把 #83 的写法套到 #82，只保留了一个重复值。
- 内层循环后忘记用标记区分是否真的重复。

## 练习顺序

建议按这个顺序刷：#83, #82。

先练保留一个，再练删除整段。两题都用有序链表连续重复这个条件，但前驱站位不同。
