---
title: 复制随机指针三步法：链表训练题解
category: 链表
summary: 复制带随机指针链表可以不用哈希表：先把复制节点插到原节点后面，再补 random，最后拆成两条链。
problem_ids: [138]
order: 111
---

# 复制随机指针三步法：链表训练题解

带随机指针链表的难点是：`random` 可能指向任意节点。哈希表做法很直观，但还有一种 $O(1)$ 额外空间的三步法。

一句话记法：**原地交织、复制 random、拆分链表。**

## 适用场景

- 链表节点除了 `Next` 还有 `Random`。
- 要深拷贝整条链表。
- 希望不用哈希表映射原节点到新节点。

如果语言指针操作不方便，哈希表版本更易读；三步法更考察链表重接能力。

## 图解思路

```text
原链: A -> B -> C
交织: A -> A' -> B -> B' -> C -> C'

若 A.random = C
则 A'.random = A.random.next = C'
```

复制节点紧跟在原节点后面，所以原节点的 random 指向谁，它的复制节点就是 `random.Next`。

## 不变量

- 第一轮后，每个原节点后面紧跟它的复制节点。
- 第二轮只补复制节点的 `Random`。
- 第三轮把交织链拆成原链和复制链。
- 拆分后原链也要恢复。

## 手写步骤

1. 遍历原链，在每个节点后插入复制节点。
2. 再遍历原链，若 `cur.Random != nil`，则 `cur.Next.Random = cur.Random.Next`。
3. 建 dummy，拆出复制节点。
4. 同时恢复原链的 `Next`。

## Go 参考实现

```go
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	for cur := head; cur != nil; {
		copy := &Node{Val: cur.Val, Next: cur.Next}
		cur.Next = copy
		cur = copy.Next
	}

	for cur := head; cur != nil; cur = cur.Next.Next {
		if cur.Random != nil {
			cur.Next.Random = cur.Random.Next
		}
	}

	dummy := &Node{}
	tail := dummy
	for cur := head; cur != nil; {
		copy := cur.Next
		next := copy.Next
		tail.Next = copy
		tail = copy
		cur.Next = next
		cur = next
	}
	return dummy.Next
}
```

## 为什么这样写

哈希表的作用是从“原节点”找到“复制节点”。三步法把这个映射藏在链表结构里：原节点的复制节点就是 `cur.Next`。因此 `cur.Random` 的复制节点就是 `cur.Random.Next`。

最后拆链时必须同时恢复原链，否则输入链表会被破坏成跳着复制节点的结构。

## 复杂度

- 时间复杂度：三次遍历，$O(n)$。
- 额外空间复杂度：$O(1)$，不计新建节点。

## 易错点

- `Random` 为空时仍然访问 `cur.Random.Next`。
- 拆链时没有恢复原链。
- 第一轮插入复制节点后，`cur` 没跳到下一个原节点。
- 第三轮拆分时把复制链尾部接回原链残留。

## 练习顺序

建议先刷 #138。

先写哈希表版本理解映射关系，再写三步法，重点体会 `原节点.Next == 复制节点` 这个临时不变量。
