---
title: 迭代器预取下一个元素：设计与数据结构训练题解
category: 设计与数据结构
summary: 用缓存槽实现 peek，不推进外部迭代器的同时返回下一个元素。
problem_ids: [284]
order: 106
---

# 迭代器预取下一个元素：设计与数据结构训练题解

`PeekingIterator` 的难点是 `peek()` 要看到下一个元素，但不能消耗它。最稳定的设计是维护一个缓存槽：

- `cache`：提前取出的下一个元素。
- `hasCache`：缓存槽是否有效。

`peek()` 确保缓存存在，然后返回缓存；`next()` 如果有缓存就消费缓存，否则直接调用底层迭代器。

## 接口模型

假设底层迭代器提供：

```go
type Iterator struct{}

func (it *Iterator) hasNext() bool
func (it *Iterator) next() int
```

封装层只负责控制缓存，不改变底层迭代器的语义。

## Go 参考实现

```go
package main

type Iterator struct{}

func (it *Iterator) hasNext() bool { return false }
func (it *Iterator) next() int     { return 0 }

type PeekingIterator struct {
	iter     *Iterator
	cache    int
	hasCache bool
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{iter: iter}
}

func (p *PeekingIterator) Peek() int {
	p.fill()
	return p.cache
}

func (p *PeekingIterator) Next() int {
	if p.hasCache {
		p.hasCache = false
		return p.cache
	}
	return p.iter.next()
}

func (p *PeekingIterator) HasNext() bool {
	return p.hasCache || p.iter.hasNext()
}

func (p *PeekingIterator) fill() {
	if !p.hasCache {
		p.cache = p.iter.next()
		p.hasCache = true
	}
}
```

## 不变量

当 `hasCache == true` 时，`cache` 保存的是逻辑上的下一个元素，并且底层迭代器已经越过它。此时 `Next()` 必须优先返回缓存，而不能再调底层 `next()`。

当 `hasCache == false` 时，封装层没有额外元素，所有状态都在底层迭代器里。

## 易错点

- 连续多次 `Peek()` 应该返回同一个值。
- `HasNext()` 要同时看缓存和底层迭代器。
- `Peek()` 通常在有下一个元素时调用；如果题目不保证，需要定义空状态行为。

## 复杂度

每个元素最多被提前读取一次，所有操作摊还 `O(1)`，额外空间 `O(1)`。
