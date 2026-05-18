---
title: 重构队列先高后低：贪心训练题解
category: 贪心
summary: 重构队列先放高个子，因为矮个子不会影响高个子的 k；高个按 k 升序插入到对应位置。
problem_ids: [406]
order: 105
---

# 重构队列先高后低：贪心训练题解

队列重构中，`[h,k]` 表示前面有 `k` 个身高大于等于 `h` 的人。先处理高个子，矮个子后插入不会影响他们的 `k`。

一句话记法：**身高降序，k 升序，按 k 插入。**

## Go 参考实现

```go
func reconstructQueue(people [][]int) [][]int {
	sort.Slice(people, func(i, j int) bool {
		if people[i][0] == people[j][0] {
			return people[i][1] < people[j][1]
		}
		return people[i][0] > people[j][0]
	})
	ans := [][]int{}
	for _, p := range people {
		ans = append(ans, nil)
		copy(ans[p[1]+1:], ans[p[1]:])
		ans[p[1]] = p
	}
	return ans
}
```

## 为什么这样写

当更高或等高的人都已经放好时，当前人的 `k` 就是它应该插入的位置。后面再插入的矮个子，不会被当前人计入“前面身高大于等于自己的人”，所以不会破坏当前人的条件。

同身高的人要按 `k` 升序，否则先插入 `k` 大的人会被后面的同高人挤偏。

## 复杂度

- 排序 $O(n \log n)$。
- 数组插入最坏 $O(n^2)$。
- 空间复杂度 $O(n)$。

## 易错点

- 按身高升序处理，矮个子会影响不了高个子的 k 推理。
- 同身高 k 没升序。
- 插入位置用 `k+1`。
- Go 插入切片时 copy 区间写错。

## 练习顺序

建议先刷 #406。
