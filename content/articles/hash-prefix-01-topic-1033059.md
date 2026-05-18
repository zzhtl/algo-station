---
title: 前缀和配合次数表：哈希与前缀训练题解
category: 哈希与前缀
summary: 和为 k 的子数组不要枚举左端点；扫描右端点时查 `prefix-k` 出现过几次，就知道有多少左端点可配。
problem_ids: [560, 525, 1248]
order: 101
---

# 前缀和配合次数表：哈希与前缀训练题解

连续子数组求和类题，核心公式是：

```text
sum(i..j) = prefix[j+1] - prefix[i]
```

要让区间和等于 `k`，只需要找之前有多少个 `prefix[i] == currentPrefix - k`。

## Go 参考实现

```go
func subarraySum(nums []int, k int) int {
	cnt := map[int]int{0: 1}
	sum, ans := 0, 0
	for _, x := range nums {
		sum += x
		ans += cnt[sum-k]
		cnt[sum]++
	}
	return ans
}
```

## 为什么这样写

扫描到当前位置时，`sum` 是右端点前缀和。任何一个旧前缀 `old` 满足 `sum - old == k`，就对应一个以当前位置结尾、和为 k 的子数组。

`cnt[0] = 1` 表示空前缀，能处理从下标 0 开始的子数组。

## 易错点

- 先 `cnt[sum]++` 再查，可能把当前前缀和自己配对。
- 忘记初始化空前缀。
- 用滑动窗口处理含负数数组，窗口单调性不成立。
- 求最长长度时，哈希表应存第一次出现下标，不是次数。

## 练习顺序

建议按这个顺序刷：#560, #525, #1248。
