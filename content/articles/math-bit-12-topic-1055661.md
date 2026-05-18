---
title: 前缀乘积处理除法：数学与位运算训练题解
category: 数学与位运算
summary: 用左侧乘积和右侧乘积替代除法，处理数组除自身以外乘积的零值边界。
problem_ids: [238]
order: 112
---

# 前缀乘积处理除法：数学与位运算训练题解

“除自身以外数组的乘积”不能使用除法，原因不只是题目限制，还有零值会让除法方案分支复杂。更稳定的思路是：答案等于左边所有数的乘积乘以右边所有数的乘积。

## 两趟扫描

第一趟从左到右：

- `ans[i]` 先保存 `i` 左边所有元素的乘积。
- `prefix` 每次乘上当前元素。

第二趟从右到左：

- `suffix` 表示 `i` 右边所有元素的乘积。
- `ans[i] *= suffix`。
- `suffix` 再乘上当前元素。

## Go 参考实现

```go
package main

func productExceptSelf(nums []int) []int {
	ans := make([]int, len(nums))

	prefix := 1
	for i, x := range nums {
		ans[i] = prefix
		prefix *= x
	}

	suffix := 1
	for i := len(nums) - 1; i >= 0; i-- {
		ans[i] *= suffix
		suffix *= nums[i]
	}

	return ans
}
```

## 为什么能自然处理 0

如果数组中有一个 `0`，只有零所在位置的左右乘积不包含这个 `0`，其他位置的左侧或右侧乘积都会包含 `0`。如果有两个以上 `0`，所有位置结果都会变成 `0`。

前后缀乘积不需要单独判断零，乘法传播会自然给出结果。

## 易错点

- `ans[i]` 第一趟保存的是“不含当前元素”的左侧乘积，所以先赋值再更新 `prefix`。
- 第二趟同理，先乘 `suffix`，再更新 `suffix`。
- 额外空间通常不把输出数组算入，因此这份写法是 `O(1)` 额外空间。

## 复杂度

两趟扫描，时间 `O(n)`，额外空间 `O(1)`。
