---
title: 质数筛批量预处理：数学与位运算训练题解
category: 数学与位运算
summary: 用埃氏筛把逐个试除变成批量标记，理解从 p*p 开始标记的原因。
problem_ids: [204]
order: 106
---

# 质数筛批量预处理：数学与位运算训练题解

如果要统计小于 `n` 的质数，逐个判断每个数是否质数会重复做大量试除。埃氏筛的想法是：从小到大遇到一个还没被标记的数 `p`，它就是质数，然后把它的倍数标记为合数。

## 为什么从 p*p 开始

标记 `p` 的倍数时，`2p, 3p, ..., (p-1)p` 已经被更小的质因子标记过了。真正第一次需要由 `p` 标记的是 `p*p`。

例如处理 `5` 时：

- `10` 已被 `2` 标记。
- `15` 已被 `3` 标记。
- `20` 已被 `2` 标记。
- 从 `25` 开始才需要动手。

## Go 参考实现

```go
package main

func countPrimes(n int) int {
	if n <= 2 {
		return 0
	}

	isComposite := make([]bool, n)
	count := 0
	for p := 2; p < n; p++ {
		if isComposite[p] {
			continue
		}
		count++
		if p*p >= n {
			continue
		}
		for multiple := p * p; multiple < n; multiple += p {
			isComposite[multiple] = true
		}
	}
	return count
}
```

## 不变量

扫描到 `p` 时，如果 `p` 还没有被标记，说明没有小于 `p` 的因子能整除它，因此它是质数。标记倍数后，后面遇到的合数都会提前被筛掉。

筛法是一种“批量排除”思路：不是证明每个数是质数，而是让合数尽早暴露出一个小因子。

## 易错点

- 题目通常要求小于 `n` 的质数，不包括 `n`。
- `0` 和 `1` 不是质数，循环从 `2` 开始。
- 如果 `n` 很大，`p*p` 可能溢出，可写成 `p <= n/p` 判断。

## 复杂度

埃氏筛时间 `O(n log log n)`，空间 `O(n)`。
