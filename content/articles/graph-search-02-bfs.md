---
title: 多源 BFS 同时扩散：搜索与图论训练题解
category: 搜索与图论
summary: 多源 BFS 把所有起点同时入队，相当于虚拟超级源点出发；第一次到达某格就是最近源点距离。
problem_ids: [994, 542, 286]
order: 102
---

# 多源 BFS 同时扩散：搜索与图论训练题解

多源 BFS 不是从每个源点各跑一次 BFS，而是把所有源点一开始都放进队列，同时向外扩散。

一句话记法：**所有源点第 0 层同时入队，谁先到就是最近距离。**

## 适用场景

- 腐烂橘子。
- 01 矩阵到最近 0 的距离。
- 墙与门到最近门的距离。
- 多个起点向外传播最短距离。

边权必须相同，通常每走一步代价为 1。

## Go 参考实现：01 矩阵

```go
func updateMatrix(mat [][]int) [][]int {
	m, n := len(mat), len(mat[0])
	q := [][2]int{}
	dist := make([][]int, m)
	for i := range dist {
		dist[i] = make([]int, n)
		for j := range dist[i] {
			dist[i][j] = -1
			if mat[i][j] == 0 {
				dist[i][j] = 0
				q = append(q, [2]int{i, j})
			}
		}
	}
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for head := 0; head < len(q); head++ {
		cur := q[head]
		for _, d := range dirs {
			r, c := cur[0]+d[0], cur[1]+d[1]
			if r < 0 || r >= m || c < 0 || c >= n || dist[r][c] != -1 {
				continue
			}
			dist[r][c] = dist[cur[0]][cur[1]] + 1
			q = append(q, [2]int{r, c})
		}
	}
	return dist
}
```

## 为什么这样写

如果对每个 1 单独找最近 0，会重复搜索很多区域。多源 BFS 把所有 0 当作同一层起点，第一次扩散到某个格子时，路径长度必然最短。

本质上可以想象有一个虚拟源点，向所有真实源点连一条长度为 0 的边。

## 复杂度

- 时间复杂度：$O(mn)$。
- 空间复杂度：$O(mn)$。

## 易错点

- 从每个目标点单独 BFS，复杂度过高。
- 源点没有全部入队。
- 入队时不标记访问，导致同一格重复入队。
- 腐烂橘子分钟数在最后一层多加一。

## 练习顺序

建议按这个顺序刷：#994, #542, #286。
