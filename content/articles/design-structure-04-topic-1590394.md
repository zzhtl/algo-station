---
title: 随机集合数组加索引表：设计与数据结构训练题解
category: 设计与数据结构
summary: 用数组支持随机访问，用哈希表支持 O(1) 定位删除，重点掌握尾部交换删除。
problem_ids: [380]
order: 104
---

# 随机集合数组加索引表：设计与数据结构训练题解

设计 `insert`、`remove`、`getRandom` 都为 `O(1)` 的集合，需要同时满足：

- 随机取元素：数组可以按随机下标访问。
- 判断存在和删除：哈希表可以快速定位。

难点是数组中间删除会移动元素。解决办法是把要删除的元素和最后一个元素交换，再删尾部。

## 不变量

- `nums` 保存当前集合所有元素。
- `index[val]` 保存 `val` 在 `nums` 中的位置。
- 删除元素时，如果用尾元素覆盖它，必须更新尾元素的新下标。

## Go 参考实现

```go
package main

import "math/rand"

type RandomizedSet struct {
	nums  []int
	index map[int]int
}

func Constructor() RandomizedSet {
	return RandomizedSet{index: map[int]int{}}
}

func (s *RandomizedSet) Insert(val int) bool {
	if _, ok := s.index[val]; ok {
		return false
	}
	s.index[val] = len(s.nums)
	s.nums = append(s.nums, val)
	return true
}

func (s *RandomizedSet) Remove(val int) bool {
	pos, ok := s.index[val]
	if !ok {
		return false
	}

	last := s.nums[len(s.nums)-1]
	s.nums[pos] = last
	s.index[last] = pos
	s.nums = s.nums[:len(s.nums)-1]
	delete(s.index, val)
	return true
}

func (s *RandomizedSet) GetRandom() int {
	return s.nums[rand.Intn(len(s.nums))]
}
```

## 为什么尾部交换有效

集合不要求保持顺序，所以删除时可以破坏数组顺序。把尾元素搬到空洞位置，只会影响尾元素的下标；同步更新 `index[last]` 后，不变量恢复。

即使删除的正好是尾元素，这套逻辑也成立：它把自己赋值给自己，然后删尾并删除哈希表记录。

## 易错点

- 先更新尾元素下标，再截断数组。
- 最后要删除 `val` 的哈希表记录。
- `GetRandom` 默认集合非空；如果题目不保证，要定义空集合行为。

## 复杂度

三个操作平均时间 `O(1)`，空间 `O(n)`。
