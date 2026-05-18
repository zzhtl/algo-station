---
title: 二分图染色判定：搜索与图论训练题解
category: 搜索与图论
summary: 二分图可以用两种颜色给节点染色，任意边两端颜色必须不同；遇到冲突说明不是二分图。
problem_ids: [785, 886]
order: 114
---

# 二分图染色判定：搜索与图论训练题解

二分图判定的核心是染色。任选一个未染色节点染成 1，然后把邻居染成相反颜色；如果某条边两端颜色相同，就失败。

一句话记法：**相邻节点必须异色。**

## 适用场景

- 判断图是否可分成两组，组内没有边。
- 可能不连通的无向图。
- 讨厌关系、冲突关系分组。

有向图依赖环问题不是二分图问题。

## Go 参考实现

```go
func isBipartite(graph [][]int) bool {
	n := len(graph)
	color := make([]int, n)
	var dfs func(int, int) bool
	dfs = func(u, c int) bool {
		color[u] = c
		for _, v := range graph[u] {
			if color[v] == 0 {
				if !dfs(v, -c) {
					return false
				}
			} else if color[v] == c {
				return false
			}
		}
		return true
	}
	for i := 0; i < n; i++ {
		if color[i] == 0 && !dfs(i, 1) {
			return false
		}
	}
	return true
}
```

## 为什么这样写

二分图等价于图中不存在奇环。染色 DFS/BFS 本质上是在尝试构造两侧集合：当前点在左侧，邻居必须在右侧，邻居的邻居又回到左侧。

如果遇到一条边连接了两个同色节点，说明约束冲突，无法二分。

## 复杂度

- 时间复杂度：$O(V+E)$。
- 空间复杂度：$O(V)$。

## 易错点

- 图不连通，只从 0 开始染色。
- 邻居已染色时不检查冲突。
- 用 true/false 表示颜色但无法区分未访问，建议用 `0/1/-1`。
- 把有向拓扑判环误当成二分图染色。

## 练习顺序

建议按这个顺序刷：#785, #886。
