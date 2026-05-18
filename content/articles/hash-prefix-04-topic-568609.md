---
title: 同余前缀归类：哈希与前缀训练题解
category: 哈希与前缀
summary: 子数组和能被 k 整除时，只需要两个前缀和模 k 相同；统计同余类出现次数即可。
problem_ids: [974, 523]
order: 104
---

# 同余前缀归类：哈希与前缀训练题解

如果 `prefix[j] - prefix[i]` 能被 `k` 整除，那么 `prefix[j] % k == prefix[i] % k`。所以问题变成统计相同余数的前缀对。

## Go 参考实现

```go
func subarraysDivByK(nums []int, k int) int {
	cnt := map[int]int{0: 1}
	sum, ans := 0, 0
	for _, x := range nums {
		sum += x
		mod := ((sum % k) + k) % k
		ans += cnt[mod]
		cnt[mod]++
	}
	return ans
}
```

## 为什么这样写

两个前缀和余数相同，它们之间的差就是 `k` 的倍数。扫描每个右端点时，之前出现过多少次相同余数，就有多少个合法左端点。

负数取模要转成非负，否则同一个同余类可能被拆成 `-1` 和 `k-1`。

## 易错点

- 忘记 `cnt[0] = 1`。
- 负数余数没有标准化。
- #523 要求子数组长度至少 2，哈希要存最早下标。
- 把整除 k 写成滑动窗口，负数或非单调时不成立。

## 练习顺序

建议按这个顺序刷：#974, #523。
