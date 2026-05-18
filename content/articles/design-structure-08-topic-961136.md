---
title: 哈希集合处理冲突：设计与数据结构训练题解
category: 设计与数据结构
summary: 用桶数组加链式存储实现简化哈希集合，理解取模定位和冲突处理。
problem_ids: [705, 706]
order: 108
---

# 哈希集合处理冲突：设计与数据结构训练题解

设计哈希集合时，核心不是“写一个 map”，而是理解两个步骤：

- 用哈希函数把 key 映射到桶下标。
- 多个 key 落到同一个桶时，用冲突结构保存它们。

刷题版可以用固定数量的桶，每个桶是一个切片或链表。

## 不变量

- `bucket = key % size` 决定 key 应该在哪个桶。
- 同一个桶里不能出现重复 key。
- `Add` 只在不存在时插入。
- `Remove` 只删除目标 key，不影响同桶其他 key。

## Go 参考实现

```go
package main

type MyHashSet struct {
	buckets [][]int
	size    int
}

func Constructor() MyHashSet {
	size := 769
	return MyHashSet{buckets: make([][]int, size), size: size}
}

func (s *MyHashSet) Add(key int) {
	b := s.bucket(key)
	for _, x := range s.buckets[b] {
		if x == key {
			return
		}
	}
	s.buckets[b] = append(s.buckets[b], key)
}

func (s *MyHashSet) Remove(key int) {
	b := s.bucket(key)
	for i, x := range s.buckets[b] {
		if x == key {
			s.buckets[b] = append(s.buckets[b][:i], s.buckets[b][i+1:]...)
			return
		}
	}
}

func (s *MyHashSet) Contains(key int) bool {
	b := s.bucket(key)
	for _, x := range s.buckets[b] {
		if x == key {
			return true
		}
	}
	return false
}

func (s *MyHashSet) bucket(key int) int {
	return key % s.size
}
```

## 为什么桶数常取质数

如果 key 有明显规律，质数桶数可以减少简单取模带来的聚集。例如大量 key 都是偶数时，偶数桶数更容易浪费一半桶位。刷题中选一个中等质数即可。

真实哈希表还会有扩容、负载因子、开放寻址等细节；这篇先把“定位桶 + 处理冲突”的骨架写稳。

## 易错点

- `Add` 前要检查重复，否则集合语义被破坏。
- `Remove` 删切片元素后要立即返回。
- 如果 key 可能为负数，桶下标要规范成非负。

## 复杂度

平均情况下操作接近 `O(1)`；最坏情况下所有 key 落到同一桶，操作退化为 `O(n)`。空间 `O(size+n)`。
