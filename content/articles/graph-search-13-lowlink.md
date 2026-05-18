---
title: 桥和割点的 lowlink：搜索与图论训练题解
category: 搜索与图论
summary: lowlink 记录一个节点通过 DFS 树边和返祖边能到达的最早时间戳；如果子节点无法回到祖先，当前边就是桥。
problem_ids: [1192]
order: 113
---

# 桥和割点的 lowlink：搜索与图论训练题解

桥是删除后会让无向图连通性增加的边。Tarjan lowlink 用一次 DFS 找出所有桥。

一句话记法：**子树回不到当前节点或更早祖先，连接它的边就是桥。**

## 核心概念

- `dfn[u]`：节点 `u` 第一次被访问的时间戳。
- `low[u]`：`u` 通过树边和最多一条返祖边能到达的最小时间戳。

对边 `u-v`，如果 `low[v] > dfn[u]`，说明 `v` 的子树回不到 `u` 或 `u` 的祖先，这条边就是桥。

## Go 参考骨架

```go
func criticalConnections(n int, connections [][]int) [][]int {
	g := make([][]int, n)
	for _, e := range connections {
		a, b := e[0], e[1]
		g[a] = append(g[a], b)
		g[b] = append(g[b], a)
	}
	dfn := make([]int, n)
	low := make([]int, n)
	time := 0
	ans := [][]int{}
	var dfs func(int, int)
	dfs = func(u, parent int) {
		time++
		dfn[u], low[u] = time, time
		for _, v := range g[u] {
			if v == parent {
				continue
			}
			if dfn[v] == 0 {
				dfs(v, u)
				low[u] = min(low[u], low[v])
				if low[v] > dfn[u] {
					ans = append(ans, []int{u, v})
				}
			} else {
				low[u] = min(low[u], dfn[v])
			}
		}
	}
	dfs(0, -1)
	return ans
}
```

## 为什么这样写

DFS 树边把图展开成树，返祖边表示子树能绕回祖先。如果 `v` 子树的 `low[v]` 仍然大于 `dfn[u]`，说明除了 `u-v` 这条边，没有任何路能把 `v` 子树接回 `u` 之前的部分。

所以删除 `u-v` 会断开图，它就是桥。

## 复杂度

- 时间复杂度：$O(V+E)$。
- 空间复杂度：$O(V+E)$。

## 易错点

- 无向图没有跳过父边，导致 low 被错误更新。
- `low[u]` 用 `low[v]` 还是 `dfn[v]` 分不清：返祖边用 `dfn[v]`。
- 图不连通时只从 0 出发，漏掉其他连通块。
- 重边场景下简单 `v == parent` 可能误跳，需要用边 id。

## 练习顺序

建议先刷 #1192。
