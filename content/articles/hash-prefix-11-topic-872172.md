---
title: 前缀异或查补集：哈希与前缀训练题解
category: 哈希与前缀
summary: 异或区间和与前缀和同构：`xor(i..j)=prefix[j+1]^prefix[i]`，要找目标就查 `prefix^target`。
problem_ids: [1442, 421]
order: 111
---

# 前缀异或查补集：哈希与前缀训练题解

异或也有前缀结构，因为相同值异或两次会抵消。

```text
xor(i..j) = pre[j+1] ^ pre[i]
```

如果区间异或要等于 `target`，旧前缀应为 `current ^ target`。

## Go 参考骨架

```go
func countXor(nums []int, target int) int {
	cnt := map[int]int{0: 1}
	xor, ans := 0, 0
	for _, x := range nums {
		xor ^= x
		ans += cnt[xor^target]
		cnt[xor]++
	}
	return ans
}
```

## 为什么这样写

因为：

```text
pre[j] ^ pre[i] = target
pre[i] = pre[j] ^ target
```

这和前缀和查 `sum-k` 完全同构，只是加减换成异或。

## 易错点

- 把异或写成加减。
- 忘记初始化空前缀异或 0。
- 异或没有大小单调性，不能用滑动窗口。
- 最大异或值 #421 更适合 Trie 或逐位贪心。

## 练习顺序

建议按这个顺序刷：#1442, #421。
