---
title: 最长连续序列起点判断：哈希与前缀训练题解
category: 哈希与前缀
summary: 最长连续序列用集合 O(1) 查找；只从 `x-1` 不存在的起点开始向后数，避免重复扫描。
problem_ids: [128]
order: 103
---

# 最长连续序列起点判断：哈希与前缀训练题解

最长连续序列不能对每个数都向后数，否则会退化。关键剪枝是：只有 `x-1` 不在集合中时，`x` 才是一个序列起点。

## Go 参考实现

```go
func longestConsecutive(nums []int) int {
	set := map[int]bool{}
	for _, x := range nums {
		set[x] = true
	}
	best := 0
	for x := range set {
		if set[x-1] {
			continue
		}
		y := x
		for set[y+1] {
			y++
		}
		if y-x+1 > best {
			best = y - x + 1
		}
	}
	return best
}
```

## 为什么这样写

如果 `x-1` 存在，说明 `x` 是某条连续序列的中间节点，从它开始数会重复。只从真正起点开始，每个数字最多被某条序列扫描一次。

这就是从暴力 $O(n^2)$ 降到 $O(n)$ 的关键，而不是哈希表本身。

## 易错点

- 排序能做但不是 O(n)。
- 不判断起点，重复向后扫描。
- 数组有重复值时没有先去重。
- `y+1` 在极值时可能溢出，工程里要注意。

## 练习顺序

建议先刷 #128。
