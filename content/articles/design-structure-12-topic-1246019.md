---
title: 支持通配符的字典树：设计与数据结构训练题解
category: 设计与数据结构
summary: 在 Trie 查询中把点号通配符转成 DFS 分支，区分确定字符和枚举子节点。
problem_ids: [211]
order: 112
---

# 支持通配符的字典树：设计与数据结构训练题解

普通 Trie 查询每一层只有一个确定分支；支持 `.` 通配符后，遇到 `.` 就要尝试当前节点的所有子节点。

设计上仍然是 Trie，只是 `Search` 从单路径查找变成 DFS。

## Go 参考实现

```go
package main

type WordDictionary struct {
	children [26]*WordDictionary
	end      bool
}

func Constructor() WordDictionary {
	return WordDictionary{}
}

func (d *WordDictionary) AddWord(word string) {
	node := d
	for i := 0; i < len(word); i++ {
		idx := word[i] - 'a'
		if node.children[idx] == nil {
			node.children[idx] = &WordDictionary{}
		}
		node = node.children[idx]
	}
	node.end = true
}

func (d *WordDictionary) Search(word string) bool {
	var dfs func(node *WordDictionary, pos int) bool
	dfs = func(node *WordDictionary, pos int) bool {
		if node == nil {
			return false
		}
		if pos == len(word) {
			return node.end
		}
		if word[pos] != '.' {
			return dfs(node.children[word[pos]-'a'], pos+1)
		}
		for _, child := range node.children {
			if child != nil && dfs(child, pos+1) {
				return true
			}
		}
		return false
	}
	return dfs(d, 0)
}
```

## 搜索不变量

`dfs(node,pos)` 表示：已经匹配了 `word[:pos]`，当前位于 Trie 的 `node`，接下来尝试匹配 `word[pos:]`。

- 普通字符：只能走对应子节点。
- `.`：枚举所有非空子节点。
- 到达模式末尾：必须检查 `node.end`，不能只看路径存在。

## 易错点

- `.` 只匹配一个字符，不是任意长度。
- DFS 到末尾时要返回 `end`，否则前缀会被误认为完整单词。
- 通配符多时复杂度会变高，这是题目允许的搜索分支成本。

## 复杂度

添加单词时间 `O(L)`。搜索最坏会枚举多个分支，最多与 Trie 中相关节点数成正比；无通配符时是 `O(L)`。
