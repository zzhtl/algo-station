---
title: 循环队列固定数组：设计与数据结构训练题解
category: 设计与数据结构
summary: 用固定数组、头指针和元素个数区分空满，训练循环下标的更新规则。
problem_ids: [622]
order: 109
---

# 循环队列固定数组：设计与数据结构训练题解

循环队列的目标是在固定容量数组上实现队列，不移动元素。关键是用取模让下标回绕。

建议维护：

- `data`：固定长度数组。
- `head`：队首下标。
- `count`：当前元素个数。

队尾插入位置可以由 `(head + count) % capacity` 算出。这样空和满用 `count` 区分，不会和 `head == tail` 混淆。

## Go 参考实现

```go
package main

type MyCircularQueue struct {
	data  []int
	head  int
	count int
}

func Constructor(k int) MyCircularQueue {
	return MyCircularQueue{data: make([]int, k)}
}

func (q *MyCircularQueue) EnQueue(value int) bool {
	if q.IsFull() {
		return false
	}
	tail := (q.head + q.count) % len(q.data)
	q.data[tail] = value
	q.count++
	return true
}

func (q *MyCircularQueue) DeQueue() bool {
	if q.IsEmpty() {
		return false
	}
	q.head = (q.head + 1) % len(q.data)
	q.count--
	return true
}

func (q *MyCircularQueue) Front() int {
	if q.IsEmpty() {
		return -1
	}
	return q.data[q.head]
}

func (q *MyCircularQueue) Rear() int {
	if q.IsEmpty() {
		return -1
	}
	tail := (q.head + q.count - 1) % len(q.data)
	return q.data[tail]
}

func (q *MyCircularQueue) IsEmpty() bool {
	return q.count == 0
}

func (q *MyCircularQueue) IsFull() bool {
	return q.count == len(q.data)
}
```

## 为什么用 count

只维护 `head` 和 `tail` 时，`head == tail` 既可能表示空，也可能表示满。可以牺牲一个空位区分，也可以额外维护 `count`。刷题时 `count` 更直观，所有操作都容易验证。

## 易错点

- 入队位置是 `(head + count) % capacity`，不是固定的 `tail+1`。
- 出队只移动 `head` 并减少 `count`，不需要清空原位置。
- 取队尾时用 `count-1`，空队列要先判断。

## 复杂度

所有操作时间 `O(1)`，空间 `O(k)`。
