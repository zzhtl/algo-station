---
title: 两个队列实现栈：栈与队列训练题解
category: 栈与队列
summary: 用队列实现栈时，可以在 push 后把前面的元素轮转到队尾，让队头始终保持最新入栈元素。
problem_ids: [225]
order: 109
---

# 两个队列实现栈：栈与队列训练题解

栈要求后进先出，队列只能先进先出。一个常见做法是：每次 push 新元素后，把它前面的旧元素全部轮转到队尾，让新元素来到队头。

一句话记法：**push 时调整顺序，pop 时直接出队。**

## 适用场景

- 用队列实现栈。
- 理解用一种线性结构模拟另一种线性结构。
- 练习把“最新元素”维护到可直接访问的位置。

## 不变量

- 主队列队头始终是栈顶。
- `push` 后要轮转旧元素。
- `pop` 直接弹队头。
- `top` 查看队头。

## Go 参考实现

```go
type MyStack struct {
	q []int
}

func Constructor() MyStack {
	return MyStack{}
}

func (s *MyStack) Push(x int) {
	s.q = append(s.q, x)
	for i := 0; i < len(s.q)-1; i++ {
		s.q = append(s.q, s.q[0])
		s.q = s.q[1:]
	}
}

func (s *MyStack) Pop() int {
	x := s.q[0]
	s.q = s.q[1:]
	return x
}

func (s *MyStack) Top() int {
	return s.q[0]
}

func (s *MyStack) Empty() bool {
	return len(s.q) == 0
}
```

## 为什么这样写

假设队列原来是 `[3,2,1]`，队头 `3` 是栈顶。push `4` 后队列先变成 `[3,2,1,4]`。把前 3 个旧元素轮转到队尾，就得到 `[4,3,2,1]`，队头又是最新元素。

这样 `pop` 和 `top` 都很简单，代价集中在 `push`。

## 复杂度

- `push` 是 $O(n)$。
- `pop/top/empty` 是 $O(1)$。
- 空间复杂度：$O(n)$。

## 易错点

- 轮转次数写成 `len(q)`，把新元素也转走。
- `top` 后错误地弹出元素。
- 两队列版本搬运后忘记交换主副队列。
- 混淆队头和队尾哪个代表栈顶。

## 练习顺序

建议先刷 #225。

再和 #232 对比：队列实现栈通常在入栈时调整顺序，栈实现队列通常在出队时按需倒栈。
