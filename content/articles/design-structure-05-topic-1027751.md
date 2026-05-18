---
title: 最小栈同步保存历史：设计与数据结构训练题解
category: 设计与数据结构
summary: 用辅助栈同步记录每一层的历史最小值，让 getMin 在 O(1) 内完成。
problem_ids: [155]
order: 105
---

# 最小栈同步保存历史：设计与数据结构训练题解

最小栈要求 `push`、`pop`、`top`、`getMin` 都是 `O(1)`。普通栈可以快速拿栈顶，但不知道当前最小值；每次扫描栈会变成 `O(n)`。

辅助栈的做法是：主栈每压入一个值，最小栈同步压入“当前这一层的最小值”。

## 不变量

- `data[i]` 是第 `i` 层真实值。
- `mins[i]` 是从底部到第 `i` 层所有值中的最小值。
- 两个栈长度始终相同。

这样 `getMin()` 只需要返回 `mins` 的栈顶。

## Go 参考实现

```go
package main

type MinStack struct {
	data []int
	mins []int
}

func Constructor() MinStack {
	return MinStack{}
}

func (s *MinStack) Push(val int) {
	s.data = append(s.data, val)
	minVal := val
	if len(s.mins) > 0 && s.mins[len(s.mins)-1] < minVal {
		minVal = s.mins[len(s.mins)-1]
	}
	s.mins = append(s.mins, minVal)
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

## 为什么要同步压入

另一种写法是只在遇到更小值时压入最小栈，但处理重复最小值时容易漏。例如压入 `2, 1, 1`，弹出一个 `1` 后当前最小值仍是 `1`。同步保存每层历史最小值，逻辑更直接，空间仍是 `O(n)`。

## 易错点

- `Pop` 时两个栈都要弹。
- 新值等于当前最小值时也要正确保留历史。
- 题目通常保证调用 `Top/GetMin/Pop` 时栈非空；如果不保证，需要额外定义错误处理。

## 复杂度

所有操作时间 `O(1)`，空间 `O(n)`。
