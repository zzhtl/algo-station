---
title: Bellman-Ford 限制边数：搜索与图论训练题解
category: 搜索与图论
summary: Bellman-Ford 按边松弛，适合有负权或限制最多走 K 条边的最短路；每轮必须基于上一轮距离拷贝更新。
problem_ids: [787]
order: 110
---

# Bellman-Ford 限制边数：搜索与图论训练题解

Bellman-Ford 不依赖非负权。它的思路是反复松弛所有边：最多走 `t` 条边的最短路，可以由最多走 `t-1` 条边的结果再接一条边得到。

一句话记法：**第 k 轮松弛，只能从上一轮距离转移。**

## 适用场景

- 有负权边的单源最短路。
- 判断负环。
- 最多经过 K 站中转。
- 边列表比邻接表更方便的题。

如果边权非负且无限制步数，Dijkstra 通常更快。

## Go 参考实现：K 站中转

```go
func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	const inf = int(1e9)
	dist := make([]int, n)
	for i := range dist {
		dist[i] = inf
	}
	dist[src] = 0
	for step := 0; step <= k; step++ {
		next := append([]int(nil), dist...)
		for _, e := range flights {
			u, v, w := e[0], e[1], e[2]
			if dist[u] != inf && dist[u]+w < next[v] {
				next[v] = dist[u] + w
			}
		}
		dist = next
	}
	if dist[dst] == inf {
		return -1
	}
	return dist[dst]
}
```

## 为什么这样写

`k` 次中转最多使用 `k+1` 条边。每一轮扩展一条边，所以循环 `k+1` 轮。

必须用 `next` 拷贝上一轮距离。如果直接原地更新，一轮内可能连续使用多条边，违反“最多本轮增加一条边”的限制。

## 复杂度

- 时间复杂度：$O(K \cdot E)$。
- 空间复杂度：$O(V)$。

## 易错点

- 原地更新 dist，导致边数限制失效。
- `k` 中转和边数 `k+1` 混淆。
- `inf + w` 溢出。
- 有负环时没做额外检测。

## 练习顺序

建议先刷 #787，再学习标准 Bellman-Ford 判负环。
