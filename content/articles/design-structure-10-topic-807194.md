---
title: 文件系统路径树：设计与数据结构训练题解
category: 设计与数据结构
summary: 把路径拆成目录段构建树，区分目录节点、文件内容和按字典序列出子节点。
problem_ids: [588, 1166]
order: 110
---

# 文件系统路径树：设计与数据结构训练题解

内存文件系统本质是一棵路径树。路径 `/a/b/c` 被拆成 `a -> b -> c`，每个节点表示一个目录或文件。

节点通常保存：

- `children`：目录下的子节点。
- `content`：如果是文件，保存文件内容。
- `isFile`：区分目录和文件。

## 结构设计

```go
package main

import (
	"sort"
	"strings"
)

type Node struct {
	children map[string]*Node
	content  string
	isFile   bool
}

type FileSystem struct {
	root *Node
}

func Constructor() FileSystem {
	return FileSystem{root: &Node{children: map[string]*Node{}}}
}
```

## 路径定位

写一个 `traverse`，根据需要决定是否创建缺失节点：

```go
func (fs *FileSystem) traverse(path string, create bool) *Node {
	node := fs.root
	if path == "/" {
		return node
	}
	parts := strings.Split(strings.Trim(path, "/"), "/")
	for _, part := range parts {
		if node.children[part] == nil {
			if !create {
				return nil
			}
			node.children[part] = &Node{children: map[string]*Node{}}
		}
		node = node.children[part]
	}
	return node
}
```

## 典型操作

```go
func (fs *FileSystem) Ls(path string) []string {
	node := fs.traverse(path, false)
	if node == nil {
		return nil
	}
	if node.isFile {
		parts := strings.Split(strings.Trim(path, "/"), "/")
		return []string{parts[len(parts)-1]}
	}
	ans := make([]string, 0, len(node.children))
	for name := range node.children {
		ans = append(ans, name)
	}
	sort.Strings(ans)
	return ans
}

func (fs *FileSystem) Mkdir(path string) {
	fs.traverse(path, true)
}

func (fs *FileSystem) AddContentToFile(filePath string, content string) {
	node := fs.traverse(filePath, true)
	node.isFile = true
	node.content += content
}

func (fs *FileSystem) ReadContentFromFile(filePath string) string {
	return fs.traverse(filePath, false).content
}
```

## 易错点

- `ls` 文件路径时返回文件名，不是返回子节点列表。
- `mkdir` 要递归创建中间目录。
- 文件内容追加不是覆盖，题目通常要求 `append`。
- 子目录列表要按字典序返回。

## 复杂度

路径定位时间与路径长度相关；`ls` 目录还需要对子节点名排序，时间 `O(c log c)`。
