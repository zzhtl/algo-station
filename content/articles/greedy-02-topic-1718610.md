---
title: 跳跃游戏维护最远边界：贪心训练题解
category: 贪心
summary: 跳跃游戏维护当前能到达的最远位置；遍历到的位置如果超过最远边界，说明前面所有选择都无法覆盖它。
problem_ids: [55, 45]
order: 102
---

# 跳跃游戏维护最远边界：贪心训练题解

跳跃游戏的关键不是决定“从哪个点跳”，而是维护到目前为止能覆盖到的最远下标。

一句话记法：**只要当前位置可达，就用它更新最远覆盖。**

## Go 参考实现：能否到达终点

```go
func canJump(nums []int) bool {
	reach := 0
	for i, x := range nums {
		if i > reach {
			return false
		}
		if i+x > reach {
			reach = i + x
		}
	}
	return true
}
```

## 最少跳跃次数

```go
func jump(nums []int) int {
	steps, end, farthest := 0, 0, 0
	for i := 0; i < len(nums)-1; i++ {
		if i+nums[i] > farthest {
			farthest = i + nums[i]
		}
		if i == end {
			steps++
			end = farthest
		}
	}
	return steps
}
```

## 为什么这样写

对 #55 来说，`[0, reach]` 是当前已经确认可达的连续区间。只要 `i <= reach`，就可以站到 `i` 上，再用 `i + nums[i]` 扩展可达范围。

对 #45 来说，`end` 是当前跳数能覆盖的边界，`farthest` 是下一跳能扩到的最远边界。走到 `end` 时必须跳一次，并把边界推进到 `farthest`。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(1)$。

## 易错点

- `reach` 初始写成 `nums[0]` 后没有处理空/单元素边界。
- #45 遍历到最后一个位置还增加一次跳数。
- `i > reach` 时还继续更新，等于从不可达位置起跳。
- 把“最远覆盖”误解成真的要跳到那个位置。

## 练习顺序

建议按这个顺序刷：#55, #45。
