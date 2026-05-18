---
title: 前缀和类支持区间查询：设计与数据结构训练题解
category: 设计与数据结构
summary: 在构造阶段预处理前缀和，让不可变数组的区间求和查询降到 O(1)。
problem_ids: [303]
order: 103
---

# 前缀和类支持区间查询：设计与数据结构训练题解

不可变数组的区间和查询，设计重点是把重复工作放到构造阶段。构造时预处理前缀和，查询时只做一次减法。

## 状态定义

令 `prefix[i]` 表示前 `i` 个元素的和，也就是 `nums[0:i]` 的和。这样：

```text
sumRange(left, right) = prefix[right+1] - prefix[left]
```

多开一个 `prefix[0] = 0`，可以统一处理从下标 `0` 开始的区间。

## Go 参考实现

```go
package main

type NumArray struct {
	prefix []int
}

func Constructor(nums []int) NumArray {
	prefix := make([]int, len(nums)+1)
	for i, x := range nums {
		prefix[i+1] = prefix[i] + x
	}
	return NumArray{prefix: prefix}
}

func (a *NumArray) SumRange(left int, right int) int {
	return a.prefix[right+1] - a.prefix[left]
}
```

## 设计取舍

这份结构适合数组不再修改的场景：

- 构造 `O(n)`。
- 每次查询 `O(1)`。
- 不支持高效单点修改。

如果题目加入 `update(index,val)`，前缀和每次更新后都要改一段，复杂度会退化；那时应考虑树状数组或线段树。

## 易错点

- `prefix` 长度应为 `n+1`，不是 `n`。
- 查询右端点要用 `right+1`，因为 `prefix` 的下标表示元素个数。
- 如果数值范围大，前缀和类型可能需要 `int64`。

## 复杂度

构造时间 `O(n)`，查询时间 `O(1)`，空间 `O(n)`。
