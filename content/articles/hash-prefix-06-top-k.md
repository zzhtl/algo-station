---
title: 频率桶找 Top K：哈希与前缀训练题解
category: 哈希与前缀
summary: Top K 高频元素先用哈希统计频率，再按频率放入桶；从高频桶往低频桶收集答案。
problem_ids: [347, 692]
order: 106
---

# 频率桶找 Top K：哈希与前缀训练题解

Top K 高频元素分两步：先统计频率，再按频率选前 K。频率最大不会超过 `n`，所以可以用桶排序。

## Go 参考实现

```go
func topKFrequent(nums []int, k int) []int {
	cnt := map[int]int{}
	for _, x := range nums {
		cnt[x]++
	}
	buckets := make([][]int, len(nums)+1)
	for x, c := range cnt {
		buckets[c] = append(buckets[c], x)
	}
	ans := []int{}
	for f := len(buckets) - 1; f >= 0 && len(ans) < k; f-- {
		for _, x := range buckets[f] {
			ans = append(ans, x)
			if len(ans) == k {
				break
			}
		}
	}
	return ans
}
```

## 为什么这样写

哈希表解决“每个元素出现几次”，桶解决“按频率排序”。频率是整数且范围有限，没必要一定用比较排序。

如果题目对同频元素有字典序要求，比如 #692，通常用堆或排序更方便。

## 复杂度

- 时间复杂度：$O(n)$，不考虑同频排序。
- 空间复杂度：$O(n)$。

## 易错点

- 只统计频率，没有处理 Top K 的选择。
- 桶大小开成不同元素个数，频率可能到 n。
- 同频顺序有要求时仍然随便返回。
- k 可能等于不同元素总数。

## 练习顺序

建议按这个顺序刷：#347, #692。
