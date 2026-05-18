---
title: 合并区间维护右端：贪心训练题解
category: 贪心
summary: 合并区间先按左端点排序；扫描时维护当前合并区间的右端，能重叠就扩展，不能重叠就结算。
problem_ids: [56, 57]
order: 106
---

# 合并区间维护右端：贪心训练题解

合并区间不是选最多，而是把所有相交区间压成不重叠区间。排序标准应该是左端点。

一句话记法：**按左端排序，重叠就扩右端，不重叠就开新区间。**

## Go 参考实现

```go
func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	ans := [][]int{}
	for _, in := range intervals {
		if len(ans) == 0 || ans[len(ans)-1][1] < in[0] {
			ans = append(ans, in)
		} else if in[1] > ans[len(ans)-1][1] {
			ans[len(ans)-1][1] = in[1]
		}
	}
	return ans
}
```

## 为什么这样写

按左端点排序后，当前区间只可能和结果中的最后一个区间重叠。若 `last.end < cur.start`，说明两者分离，可以直接开启新区间。否则合并，并把右端点更新为更大的那个。

这里不要按右端排序。右端排序适合区间调度，左端排序适合合并。

## 复杂度

- 时间复杂度：$O(n \log n)$。
- 空间复杂度：输出空间 $O(n)$。

## 易错点

- 区间调度和区间合并的排序标准混用。
- 重叠条件边界写错，`[1,4]` 和 `[4,5]` 应合并。
- 合并时只覆盖右端，没有取最大值。
- 插入区间 #57 没先处理新区间前后的三段。

## 练习顺序

建议按这个顺序刷：#56, #57。
