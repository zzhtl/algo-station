---
title: 循环队列固定容量：栈与队列训练题解
category: 栈与队列
summary: 循环队列用数组和取模复用空间；常见设计是额外留一个空位，用 `front == rear` 表示空，`(rear+1)%cap == front` 表示满。
problem_ids: [622]
order: 114
---

# 循环队列固定容量：栈与队列训练题解

循环队列的目标是在固定数组里实现队列，不随着出队不断搬移元素。通过取模，尾部到数组末尾后可以绕回开头。

一句话记法：**下标前进都取模；留一个空位区分空和满。**

## 适用场景

- 固定容量队列。
- 高频入队出队，不想移动数组元素。
- 需要判断空和满。

也可以用 `size` 字段区分空满；本文用“浪费一个槽位”的写法，判断更简单。

## 不变量

- `front` 指向队头元素。
- `rear` 指向下一个可写位置。
- `front == rear` 表示空。
- `(rear + 1) % cap == front` 表示满。
- 实际可存元素个数是 `cap - 1`。

如果题目要求容量为 `k`，底层数组长度要开 `k + 1`。

## Go 参考实现

```go
type MyCircularQueue struct {
	data        []int
	front, rear int
}

func Constructor(k int) MyCircularQueue {
	return MyCircularQueue{data: make([]int, k+1)}
}

func (q *MyCircularQueue) EnQueue(value int) bool {
	if q.IsFull() {
		return false
	}
	q.data[q.rear] = value
	q.rear = (q.rear + 1) % len(q.data)
	return true
}

func (q *MyCircularQueue) DeQueue() bool {
	if q.IsEmpty() {
		return false
	}
	q.front = (q.front + 1) % len(q.data)
	return true
}

func (q *MyCircularQueue) Front() int {
	if q.IsEmpty() {
		return -1
	}
	return q.data[q.front]
}

func (q *MyCircularQueue) Rear() int {
	if q.IsEmpty() {
		return -1
	}
	idx := (q.rear - 1 + len(q.data)) % len(q.data)
	return q.data[idx]
}

func (q *MyCircularQueue) IsEmpty() bool {
	return q.front == q.rear
}

func (q *MyCircularQueue) IsFull() bool {
	return (q.rear+1)%len(q.data) == q.front
}
```

## 为什么这样写

如果不额外维护 `size`，`front == rear` 既可能表示空，也可能表示满。留一个空位后，满的状态提前一格发生，空和满就能区分。

`rear` 指向下一个可写位置，而不是最后一个元素。这样入队先写 `data[rear]`，再推进 `rear`；取队尾元素时才需要看 `rear-1`。

## 复杂度

- 所有操作都是 $O(1)$。
- 空间复杂度：$O(k)$。

## 易错点

- 底层数组只开 `k`，但又使用留空位判满，实际容量变成 `k-1`。
- `rear` 的语义混乱，一会儿表示尾元素，一会儿表示下一个写入位置。
- `Rear()` 计算 `rear-1` 时没有处理负数。
- 忘记所有下标移动都要取模。

## 练习顺序

建议先刷 #622。

做完后再写一个带 `size` 字段的版本，对比两种空满判断方式。
