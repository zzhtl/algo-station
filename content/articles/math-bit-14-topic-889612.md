---
title: 模运算保持非负：数学与位运算训练题解
category: 数学与位运算
summary: 统一处理负数取模、循环下标和差值取模，避免语言取余规则造成边界错误。
problem_ids: [974, 1590]
order: 114
---

# 模运算保持非负：数学与位运算训练题解

很多语言里的 `%` 是取余，不是严格数学意义上的非负取模。负数参与时，结果可能仍然是负数。为了让余数落在 `[0, mod)`，常用写法是：

```text
((x % mod) + mod) % mod
```

这在前缀和取模、循环数组下标、差值取模里都很关键。

## 前缀和同余

子数组和能被 `k` 整除，等价于两个前缀和模 `k` 的余数相同：

```text
(prefix[j] - prefix[i]) % k == 0
=> prefix[j] % k == prefix[i] % k
```

如果数组里有负数，前缀余数要规范成非负值。

## Go 参考实现

```go
package main

func subarraysDivByK(nums []int, k int) int {
	count := map[int]int{0: 1}
	prefix := 0
	ans := 0

	for _, x := range nums {
		prefix += x
		rem := ((prefix % k) + k) % k
		ans += count[rem]
		count[rem]++
	}

	return ans
}
```

## 差值取模

有些题要找需要删除的子数组，使剩余和能被 `p` 整除。设总余数为 `need`，当前前缀余数为 `cur`，需要找之前的前缀余数：

```text
target = (cur - need + p) % p
```

这里加 `p` 是为了避免 `cur - need` 为负。

## 易错点

- 取模前要确认模数为正。
- 负数规范化不要只写 `x%mod + mod`，因为正数时可能超过范围；完整写法再 `% mod` 一次。
- 前缀同余题通常要先放入余数 `0` 的计数或下标，代表空前缀。

## 复杂度

前缀和配合哈希表，时间 `O(n)`，空间 `O(k)` 或 `O(n)`，取决于余数范围和题目数据。
