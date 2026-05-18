---
title: 区间 DP 枚举分割点：动态规划训练题解
category: 动态规划
summary: 区间 DP 的状态是 `[l,r]` 上的答案，转移通常枚举中间分割点；填表顺序按区间长度从短到长。
problem_ids: [5, 516, 312, 1547]
order: 105
---

# 区间 DP 枚举分割点：动态规划训练题解

区间 DP 处理的是一段连续区间的最优值或可行性。当前大区间依赖更短的小区间，所以填表要按区间长度递增。

一句话记法：**先算短区间，再算长区间；转移枚举中间断点。**

## 适用场景

- 最长回文子串/子序列。
- 戳气球。
- 切木棍最小成本。
- 石子合并类问题。

如果状态天然是前缀或后缀，通常不是区间 DP。

## 状态与顺序

常见状态：

```text
dp[l][r] = 区间 l..r 上的答案
```

填表顺序：

```text
for length from 1..n:
    for l:
        r = l + length - 1
```

这样 `dp[l][k]`、`dp[k][r]`、`dp[l+1][r-1]` 都已经算过。

## Go 参考实现：最长回文子串

```go
func longestPalindrome(s string) string {
	n := len(s)
	dp := make([][]bool, n)
	for i := range dp {
		dp[i] = make([]bool, n)
	}
	start, best := 0, 0
	for length := 1; length <= n; length++ {
		for l := 0; l+length-1 < n; l++ {
			r := l + length - 1
			dp[l][r] = s[l] == s[r] && (length <= 2 || dp[l+1][r-1])
			if dp[l][r] && length > best {
				start, best = l, length
			}
		}
	}
	return s[start : start+best]
}
```

## 为什么这样写

回文区间 `[l,r]` 是否成立，依赖两端字符是否相等，以及内部 `[l+1,r-1]` 是否回文。内部区间更短，所以必须先算短区间。

戳气球这类题则通常枚举最后一个被处理的位置 `k`，让左右区间互不影响。

## 复杂度

- 常见区间 DP 是 $O(n^2)$ 或 $O(n^3)$。
- 空间复杂度通常 $O(n^2)$。

## 易错点

- 按 `l` 从小到大、`r` 从小到大乱填，依赖状态还没算。
- 长度为 1、2 的边界没处理。
- 枚举分割点时区间开闭混乱。
- 回文子串和回文子序列状态转移混用。

## 练习顺序

建议按这个顺序刷：#5, #516, #312, #1547。
