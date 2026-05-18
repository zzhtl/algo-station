---
title: Floyd 处理全源最短路：搜索与图论训练题解
category: 搜索与图论
summary: Floyd 用 `k` 作为中转点逐步放开限制，适合点数较小的全源最短路和传递闭包。
problem_ids: [1334, 399, 1462]
order: 111
---

# Floyd 处理全源最短路：搜索与图论训练题解

Floyd-Warshall 解决所有点对之间的最短路。它的核心是逐步允许更多中转点。

一句话记法：**枚举中转点 k，看 i 到 j 能否经 k 变短。**

## 适用场景

- 点数较小，通常 `n <= 100` 或几百。
- 要查询很多点对最短路。
- 传递闭包、课程先修可达性。
- 图可以有负权，但不能有负环。

点数很大时，Floyd 的 $O(n^3)$ 会吃不消。

## Go 参考骨架

```go
func floyd(n int, edges [][]int) [][]int {
	const inf = int(1e9)
	dist := make([][]int, n)
	for i := range dist {
		dist[i] = make([]int, n)
		for j := range dist[i] {
			if i == j {
				dist[i][j] = 0
			} else {
				dist[i][j] = inf
			}
		}
	}
	for _, e := range edges {
		u, v, w := e[0], e[1], e[2]
		dist[u][v] = min(dist[u][v], w)
	}
	for k := 0; k < n; k++ {
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				if dist[i][k] != inf && dist[k][j] != inf {
					dist[i][j] = min(dist[i][j], dist[i][k]+dist[k][j])
				}
			}
		}
	}
	return dist
}
```

## 为什么 k 在最外层

`k` 表示当前允许使用编号 `0..k` 的点作为中转。计算 `dist[i][j]` 时，要么不经过 `k`，要么经过 `k`，即 `dist[i][k] + dist[k][j]`。

如果把 `k` 放到内层，状态含义会被破坏。

## 复杂度

- 时间复杂度：$O(n^3)$。
- 空间复杂度：$O(n^2)$。

## 易错点

- 三层循环顺序错，把 `k` 放内层。
- `inf + inf` 溢出。
- 无向图忘记同时设置 `dist[u][v]` 和 `dist[v][u]`。
- 多条边没有取最小权重。

## 练习顺序

建议按这个顺序刷：#1334, #1462, #399。
