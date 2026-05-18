---
title: 最小栈保存历史最小值：栈与队列训练题解
category: 栈与队列
summary: 最小栈要让 `getMin` 为 O(1)，做法是每次入栈时同时保存“当前栈的最小值”。
problem_ids: [155]
order: 105
---

# 最小栈保存历史最小值：栈与队列训练题解

最小栈不能在 `getMin` 时扫描全栈。要做到 $O(1)$，必须在入栈时就把历史最小值保存下来。

一句话记法：**每个元素入栈时，顺手记录它入栈后的栈内最小值。**

## 适用场景

- 栈支持 `push`、`pop`、`top`。
- 还要支持 `getMin`，并要求常数时间。
- 操作顺序在线发生，不能离线预处理。

常见做法有两个栈，或者一个栈里存 `(value, currentMin)`。

## 图解思路

```text
push 3 -> (3,3)
push 5 -> (5,3)
push 2 -> (2,2)
pop    -> 栈顶回到 (5,3)，最小值自动恢复为 3
```

历史最小值跟随栈帧一起进出。

## 不变量

- 数据栈保存真实值。
- 最小值栈栈顶保存当前所有元素的最小值。
- 两个栈长度同步，或单栈每个元素携带当前最小值。
- `pop` 时最小值状态也必须一起回退。

## Go 参考实现

```go
type MinStack struct {
	data []int
	mins []int
}

func Constructor() MinStack {
	return MinStack{}
}

func (s *MinStack) Push(val int) {
	s.data = append(s.data, val)
	if len(s.mins) == 0 || val < s.mins[len(s.mins)-1] {
		s.mins = append(s.mins, val)
	} else {
		s.mins = append(s.mins, s.mins[len(s.mins)-1])
	}
}

func (s *MinStack) Pop() {
	s.data = s.data[:len(s.data)-1]
	s.mins = s.mins[:len(s.mins)-1]
}

func (s *MinStack) Top() int {
	return s.data[len(s.data)-1]
}

func (s *MinStack) GetMin() int {
	return s.mins[len(s.mins)-1]
}
```

## 为什么这样写

如果只在最小值变化时压入辅助栈，也可以，但处理重复最小值时要更小心。同步长度的写法更直观：每个位置都记录“截至当前位置的最小值”，弹出时天然恢复上一个状态。

例如 `[3,2,2]` 连续弹出两个 `2` 时，最小值应该先仍为 `2`，再恢复为 `3`。同步最小栈不会漏掉重复最小值。

## 复杂度

- `push/pop/top/getMin` 都是 $O(1)$。
- 空间复杂度：$O(n)$。

## 易错点

- 只保存一个全局最小值，pop 掉最小值后不知道如何恢复。
- 辅助栈不处理重复最小值。
- `pop` 只弹数据栈，不弹最小栈。
- 空栈调用不按题目约束处理。

## 练习顺序

建议先刷 #155。

复盘时写两种版本：同步最小栈，以及“只在变小或相等时入辅助栈”的压缩版本。
