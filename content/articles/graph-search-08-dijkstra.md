---
title: Dijkstra 处理非负权：搜索与图论训练题解
category: 搜索与图论
summary: Dijkstra 用小根堆每次取当前距离最小的点；非负权保证点弹出时距离已经确定。
problem_ids: [743, 1631, 1514, 1976]
order: 108
---

# Dijkstra 处理非负权：搜索与图论训练题解

BFS 只适用于边权相同的图。边权非负但不相同时，要用 Dijkstra。

一句话记法：**小根堆弹出的最小距离，一旦不是过期项，就已经确定。**

## 适用场景

- 单源最短路。
- 边权非负。
- 图可能稀疏，适合邻接表 + 堆。
- 网格最小代价路径也可建模成图。

有负权边不能用 Dijkstra。

## Go 参考骨架

```go
func dijkstra(n int, graph [][][2]int, src int) []int {
	const inf = int(1e18)
	dist := make([]int, n)
	for i := range dist {
		dist[i] = inf
	}
	dist[src] = 0
	h := &MinHeap{{0, src}}
	heap.Init(h)
	for h.Len() > 0 {
		item := heap.Pop(h).([2]int)
		d, u := item[0], item[1]
		if d > dist[u] {
			continue
		}
		for _, e := range graph[u] {
			v, w := e[0], e[1]
			if d+w < dist[v] {
				dist[v] = d + w
				heap.Push(h, [2]int{dist[v], v})
			}
		}
	}
	return dist
}
```

## 为什么这样写

非负权保证：当前堆里距离最小的点 `u`，不可能之后通过别的未确定点绕一圈得到更短距离。因为绕路只会增加非负边权。

堆中可能有同一个节点的旧距离，弹出时用 `if d > dist[u]` 跳过即可，这叫懒删除。

## 复杂度

- 时间复杂度：$O((V+E)\log V)$。
- 空间复杂度：$O(V+E)$。

## 易错点

- 有负权边还使用 Dijkstra。
- 没有跳过过期堆项。
- `inf` 太大相加溢出。
- Go 需要自己实现 `heap.Interface`。

## 练习顺序

建议按这个顺序刷：#743, #1631, #1514, #1976。
