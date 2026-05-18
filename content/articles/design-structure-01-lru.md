---
title: LRU 哈希表加双链表：设计与数据结构训练题解
category: 设计与数据结构
summary: 用哈希表定位节点、双链表维护新旧顺序，拆清 LRU 每个操作的同步不变量。
problem_ids: [146]
order: 101
---

# LRU 哈希表加双链表：设计与数据结构训练题解

LRU 缓存要求 `get` 和 `put` 都是 `O(1)`，同时容量满时淘汰最久未使用的键。单独用哈希表能 `O(1)` 查值，但不知道谁最旧；单独用链表能维护顺序，但查找要 `O(n)`。所以必须组合：

- 哈希表：`key -> 节点指针`，负责快速定位。
- 双链表：从头到尾表示“最近使用 -> 最久未使用”。

## 不变量

- 链表头部是最近使用节点。
- 链表尾部是最久未使用节点。
- 哈希表中的每个 key 都指向链表中的真实节点。
- 删除或移动节点时，哈希表和链表必须同步更新。

`get(key)` 命中后，要把节点移动到头部；`put(key,value)` 更新旧 key 时也要移动到头部；插入新 key 超容量时，删除尾部节点。

## Go 参考实现

```go
package main

type Node struct {
	key, value int
	prev, next *Node
}

type LRUCache struct {
	capacity int
	cache    map[int]*Node
	head     *Node
	tail     *Node
}

func Constructor(capacity int) LRUCache {
	head, tail := &Node{}, &Node{}
	head.next = tail
	tail.prev = head
	return LRUCache{capacity: capacity, cache: map[int]*Node{}, head: head, tail: tail}
}

func (c *LRUCache) Get(key int) int {
	node, ok := c.cache[key]
	if !ok {
		return -1
	}
	c.moveToFront(node)
	return node.value
}

func (c *LRUCache) Put(key int, value int) {
	if node, ok := c.cache[key]; ok {
		node.value = value
		c.moveToFront(node)
		return
	}

	node := &Node{key: key, value: value}
	c.cache[key] = node
	c.addAfterHead(node)
	if len(c.cache) > c.capacity {
		old := c.tail.prev
		c.remove(old)
		delete(c.cache, old.key)
	}
}

func (c *LRUCache) moveToFront(node *Node) {
	c.remove(node)
	c.addAfterHead(node)
}

func (c *LRUCache) addAfterHead(node *Node) {
	node.prev = c.head
	node.next = c.head.next
	c.head.next.prev = node
	c.head.next = node
}

func (c *LRUCache) remove(node *Node) {
	node.prev.next = node.next
	node.next.prev = node.prev
}
```

## 为什么用哨兵节点

哨兵 `head` 和 `tail` 让插入、删除都不需要判断空链表、头节点、尾节点这些边界。所有真实节点都在两个哨兵之间，移动节点只需要“先摘下，再插到头后”。

## 易错点

- 更新已有 key 时也算访问，必须移动到头部。
- 淘汰尾部节点后要同时 `delete` 哈希表。
- 链表节点必须保存 key，否则淘汰时不知道删哈希表里的哪个键。

## 复杂度

`get` 和 `put` 都是 `O(1)`，空间 `O(capacity)`。
