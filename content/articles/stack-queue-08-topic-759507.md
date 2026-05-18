---
title: 两个栈实现队列：栈与队列训练题解
category: 栈与队列
summary: 入队栈负责接收新元素，出队栈负责弹出旧元素；只有出队栈为空时，才把入队栈整体倒过去。
problem_ids: [232]
order: 108
---

# 两个栈实现队列：栈与队列训练题解

队列是先进先出，栈是后进先出。两个栈配合时，一次倒栈可以把顺序反回来。

一句话记法：**push 进 in 栈；pop/peek 从 out 栈取；out 空了才搬运。**

## 适用场景

- 用栈实现队列。
- 理解摊还复杂度。
- 需要把输入顺序反转一次后输出。

不要每次 `push` 都倒栈，那样复杂度会变差且代码更乱。

## 不变量

- `out` 栈顶是当前队头。
- 新元素都进入 `in` 栈。
- 只有 `out` 为空时，才把 `in` 全部倒入 `out`。
- 队列内容等于 `out` 从栈顶到栈底，再接 `in` 从栈底到栈顶。

## Go 参考实现

```go
type MyQueue struct {
	in  []int
	out []int
}

func Constructor() MyQueue {
	return MyQueue{}
}

func (q *MyQueue) Push(x int) {
	q.in = append(q.in, x)
}

func (q *MyQueue) move() {
	if len(q.out) > 0 {
		return
	}
	for len(q.in) > 0 {
		n := len(q.in) - 1
		q.out = append(q.out, q.in[n])
		q.in = q.in[:n]
	}
}

func (q *MyQueue) Pop() int {
	q.move()
	n := len(q.out) - 1
	x := q.out[n]
	q.out = q.out[:n]
	return x
}

func (q *MyQueue) Peek() int {
	q.move()
	return q.out[len(q.out)-1]
}

func (q *MyQueue) Empty() bool {
	return len(q.in) == 0 && len(q.out) == 0
}
```

## 为什么这样写

元素从 `in` 倒到 `out` 时，顺序被反转。最早进入 `in` 的元素会变成 `out` 栈顶，所以可以先出队。

每个元素最多进入 `in` 一次、从 `in` 搬到 `out` 一次、从 `out` 弹出一次，因此虽然某次 `pop` 可能搬很多元素，摊还下来每个操作仍是 $O(1)$。

## 复杂度

- `push` 是 $O(1)$。
- `pop/peek` 摊还 $O(1)$，单次最坏 $O(n)$。
- 空间复杂度：$O(n)$。

## 易错点

- 每次 `push` 都倒栈，复杂度高。
- `out` 不为空时仍然搬运，破坏队列顺序。
- `peek` 忘记先 `move`。
- `Empty` 只判断一个栈。

## 练习顺序

建议先刷 #232。

复盘时重点解释摊还复杂度，而不是只背两个栈。
