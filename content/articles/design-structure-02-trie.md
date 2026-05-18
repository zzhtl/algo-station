---
title: Trie 节点保存分支：设计与数据结构训练题解
category: 设计与数据结构
summary: 用路径表示前缀、终止标记表示完整单词，训练 Trie 的插入、查找和前缀查询。
problem_ids: [208]
order: 102
---

# Trie 节点保存分支：设计与数据结构训练题解

Trie 适合处理大量字符串的前缀查询。它把字符串拆成字符路径：从根走到某个节点，路径上的字符就是一个前缀。

每个节点要保存两类信息：

- `children`：下一字符到子节点的映射。
- `end`：是否有单词在这个节点结束。

## 不变量

- 根节点不表示任何字符。
- 从根到某节点的路径唯一表示一个前缀。
- `end == true` 表示这个前缀本身是完整单词。
- 查询单词必须走完整条路径并检查 `end`；查询前缀只需要路径存在。

## Go 参考实现

```go
package main

type Trie struct {
	children [26]*Trie
	end      bool
}

func Constructor() Trie {
	return Trie{}
}

func (t *Trie) Insert(word string) {
	node := t
	for i := 0; i < len(word); i++ {
		idx := word[i] - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &Trie{}
		}
		node = node.children[idx]
	}
	node.end = true
}

func (t *Trie) Search(word string) bool {
	node := t.find(word)
	return node != nil && node.end
}

func (t *Trie) StartsWith(prefix string) bool {
	return t.find(prefix) != nil
}

func (t *Trie) find(s string) *Trie {
	node := t
	for i := 0; i < len(s); i++ {
		idx := s[i] - 'a'
		if node.children[idx] == nil {
			return nil
		}
		node = node.children[idx]
	}
	return node
}
```

## 为什么 search 和 startsWith 不一样

`"app"` 是 `"apple"` 的前缀，但不一定被插入为完整单词。因此：

- `StartsWith("app")` 只检查路径是否存在。
- `Search("app")` 还要检查路径末尾节点的 `end`。

这个区别是 Trie 题最常见的漏点。

## 易错点

- 如果字符集不是小写字母，`[26]*Trie` 要换成 `map[byte]*Trie` 或 `map[rune]*Trie`。
- 插入结束后才标记 `end`，不要在中间节点误标。
- 空字符串如果题目允许，需要明确根节点是否能作为结束节点。

## 复杂度

插入、查找、前缀查询都与字符串长度成正比，时间 `O(L)`；空间与总字符数相关。
