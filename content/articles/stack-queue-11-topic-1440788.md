---
title: 柱状图延迟结算面积：栈与队列训练题解
category: 栈与队列
summary: 柱状图最大矩形用递增栈延迟结算：当遇到更矮柱子时，被弹出的柱子终于确定了右边界。
problem_ids: [84]
order: 111
---

# 柱状图延迟结算面积：栈与队列训练题解

柱状图最大矩形的关键转换是：枚举每根柱子作为矩形的最矮高度，找到它左右第一个更矮的位置。

一句话记法：**柱子被弹出时，左右边界才同时确定。**

## 适用场景

- 柱状图最大矩形。
- 需要找左右第一个更小边界。
- 可以通过哨兵强制结算剩余元素。

## 图解思路

```text
leftLess ... [height[top]] ... currentLower

top 被弹出时：
右边界 = 当前 i
左边界 = 弹出后新的栈顶
宽度 = i - left - 1
```

栈保持递增高度，遇到更矮柱子就结算一批更高柱子。

## Go 参考实现

```go
func largestRectangleArea(heights []int) int {
	heights = append(heights, 0)
	st := []int{}
	ans := 0
	for i, h := range heights {
		for len(st) > 0 && heights[st[len(st)-1]] > h {
			top := st[len(st)-1]
			st = st[:len(st)-1]
			left := -1
			if len(st) > 0 {
				left = st[len(st)-1]
			}
			area := heights[top] * (i - left - 1)
			if area > ans {
				ans = area
			}
		}
		st = append(st, i)
	}
	return ans
}
```

## 为什么这样写

一个柱子只要右边还没出现更矮柱，它作为最矮高度的矩形就还能继续向右扩，所以不能结算。直到当前柱子更矮，才知道它右边界就是当前位置。

弹出后新的栈顶，是左边第一个比它更矮的位置。两个边界之间的宽度就是 `i - left - 1`。

末尾追加高度 `0` 的哨兵，是为了让所有剩余柱子都被弹出结算。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(n)$。

## 易错点

- 忘记哨兵，末尾递增柱子没有结算。
- 宽度写成 `i - left`，多算一格。
- 弹出后没有重新取左边界。
- 栈里存高度，算宽度时缺下标。

## 练习顺序

建议先刷 #84，再回看单调递增栈找左边界。

这题是单调栈从“找边界”升级到“弹栈时结算贡献”的代表题。
