---
title: 辗转相除求最大公约数：数学与位运算训练题解
category: 数学与位运算
summary: 用 gcd(a,b)=gcd(b,a%b) 缩小问题，训练数论题里的不变量转换。
problem_ids: [1979, 1071]
order: 105
---

# 辗转相除求最大公约数：数学与位运算训练题解

最大公约数的关键等式是：

```text
gcd(a, b) = gcd(b, a % b)
```

原因是：能同时整除 `a` 和 `b` 的数，也一定能整除 `a - q*b`，也就是 `a % b`；反过来也成立。

## Go 参考实现

```go
package main

func gcd(a, b int) int {
	if a < 0 {
		a = -a
	}
	if b < 0 {
		b = -b
	}
	for b != 0 {
		a, b = b, a%b
	}
	return a
}
```

## 不变量

循环过程中，`gcd(a,b)` 的值始终不变，但第二个参数会不断变小。直到 `b == 0`，此时 `gcd(a,0) = a`，答案就出来了。

这个不变量很适合迁移：

- 多个数字的 gcd：从左到右累积 `g = gcd(g, nums[i])`。
- 字符串 gcd：先验证 `s1+s2 == s2+s1`，再取长度的 gcd。

## 字符串最大公因子

```go
func gcdOfStrings(str1 string, str2 string) string {
	if str1+str2 != str2+str1 {
		return ""
	}
	length := gcd(len(str1), len(str2))
	return str1[:length]
}
```

两个字符串能由同一个模式重复得到时，拼接顺序才会相同；模式长度就是两个长度的最大公约数。

## 易错点

- `a % b` 前要保证 `b != 0`。
- 语言对负数取模的定义不同，通用写法先转非负。
- 字符串 gcd 不能只看长度，还要先验证重复模式一致。

## 复杂度

数值 gcd 时间 `O(log min(a,b))`，空间 `O(1)`。字符串版本主要成本是拼接比较，时间 `O(n+m)`。
