---
title: 分发糖果左右两遍：贪心训练题解
category: 贪心
summary: 分发糖果的约束来自左右两个方向；先从左到右满足左邻居，再从右到左满足右邻居。
problem_ids: [135]
order: 104
---

# 分发糖果左右两遍：贪心训练题解

每个孩子至少一个糖果，评分更高的孩子要比相邻评分低的孩子糖果更多。这个约束有左右两个方向，单次扫描很难同时满足。

一句话记法：**左规则扫一遍，右规则再扫一遍，取最大值。**

## Go 参考实现

```go
func candy(ratings []int) int {
	n := len(ratings)
	candies := make([]int, n)
	for i := range candies {
		candies[i] = 1
	}
	for i := 1; i < n; i++ {
		if ratings[i] > ratings[i-1] {
			candies[i] = candies[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if ratings[i] > ratings[i+1] && candies[i] <= candies[i+1] {
			candies[i] = candies[i+1] + 1
		}
	}
	sum := 0
	for _, x := range candies {
		sum += x
	}
	return sum
}
```

## 为什么这样写

从左到右只能保证：如果 `ratings[i] > ratings[i-1]`，那么 `candies[i] > candies[i-1]`。但它无法处理右边更低的情况。

第二遍从右往左补右侧约束。不能直接覆盖，要取能同时满足左右约束的较大值。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(n)$。

## 易错点

- 只扫一遍，无法处理下降坡。
- 第二遍直接赋值，破坏第一遍已经满足的左约束。
- 相等评分不需要更多糖果。
- 忘记每人至少一个糖果。

## 练习顺序

建议先刷 #135。
