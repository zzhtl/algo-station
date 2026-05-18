---
title: 随机化避免最坏情况：数学与位运算训练题解
category: 数学与位运算
summary: 用随机枢轴降低快速选择退化风险，理解期望复杂度和确定性边界。
problem_ids: [215, 912]
order: 113
---

# 随机化避免最坏情况：数学与位运算训练题解

快速选择和快速排序都依赖枢轴划分。如果每次都选到极端枢轴，复杂度会退化；随机化的目的，是让输入数据很难稳定制造最坏划分。

以数组第 K 大为例，可以把它转成第 `len(nums)-k` 小，然后用随机快速选择。

## 划分不变量

一次 partition 之后，枢轴下标 `p` 满足：

- `p` 左侧元素都 `<= pivot`。
- `p` 右侧元素都 `> pivot`。
- 如果 `p == target`，枢轴就是答案。
- 如果 `p < target`，答案在右侧；否则在左侧。

## Go 参考实现

```go
package main

import "math/rand"

func findKthLargest(nums []int, k int) int {
	target := len(nums) - k
	left, right := 0, len(nums)-1

	for left <= right {
		pivotIndex := left + rand.Intn(right-left+1)
		p := partition(nums, left, right, pivotIndex)
		if p == target {
			return nums[p]
		}
		if p < target {
			left = p + 1
		} else {
			right = p - 1
		}
	}
	return -1
}

func partition(nums []int, left, right, pivotIndex int) int {
	pivot := nums[pivotIndex]
	nums[pivotIndex], nums[right] = nums[right], nums[pivotIndex]

	store := left
	for i := left; i < right; i++ {
		if nums[i] <= pivot {
			nums[store], nums[i] = nums[i], nums[store]
			store++
		}
	}
	nums[store], nums[right] = nums[right], nums[store]
	return store
}
```

## 随机化解决了什么

随机化不改变最坏复杂度的理论上界，但把最坏情况从“某些输入形状必然触发”变成“随机过程极小概率触发”。因此快速选择的期望时间是 `O(n)`。

如果题目或系统要求严格最坏时间保证，就要用堆、排序，或更复杂的 BFPRT；刷题中随机快速选择通常足够。

## 易错点

- 第 K 大对应升序下标 `len(nums)-k`，不是 `k-1`。
- partition 后只递归或循环一边，快速选择不是完整排序。
- 随机数用于选择枢轴即可，不需要打乱整个数组。

## 复杂度

期望时间 `O(n)`，最坏时间 `O(n^2)`；原地划分，额外空间 `O(1)`。
