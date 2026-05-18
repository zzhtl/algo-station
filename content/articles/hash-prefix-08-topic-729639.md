---
title: 集合去重再扫描：哈希与前缀训练题解
category: 哈希与前缀
summary: 哈希集合适合回答“某个值是否存在”；先去重，再只从有意义的起点扫描，可以避免重复工作。
problem_ids: [217, 128, 349]
order: 108
---

# 集合去重再扫描：哈希与前缀训练题解

哈希集合最常见的作用是去重和 O(1) 存在性查询。很多题的优化点不是 set 本身，而是只扫描必要的候选。

## Go 参考实现：存在重复元素

```go
func containsDuplicate(nums []int) bool {
	seen := map[int]bool{}
	for _, x := range nums {
		if seen[x] {
			return true
		}
		seen[x] = true
	}
	return false
}
```

## 交集

```go
func intersection(a []int, b []int) []int {
	set := map[int]bool{}
	for _, x := range a {
		set[x] = true
	}
	ansSet := map[int]bool{}
	for _, x := range b {
		if set[x] {
			ansSet[x] = true
		}
	}
	ans := []int{}
	for x := range ansSet {
		ans = append(ans, x)
	}
	return ans
}
```

## 为什么这样写

集合会丢弃重复值，所以适合“不关心次数”的题。如果题目关心出现次数，比如两个数组交集 II，就要用计数表。

最长连续序列中，set 还提供了 `x-1`、`x+1` 是否存在的 O(1) 查询。

## 易错点

- 题目需要次数，却用了 set。
- 把 `slice contains` 当成哈希查询，复杂度仍是 O(n)。
- 遍历 map 输出顺序不稳定。
- 去重后忘记题目是否要求保留原顺序。

## 练习顺序

建议按这个顺序刷：#217, #349, #128。
