---
title: 状态压缩 BFS：搜索与图论训练题解
category: 搜索与图论
summary: 当“位置”不足以描述搜索状态时，把钥匙集合、访问集合等附加信息压进状态，一起放入 BFS 队列。
problem_ids: [864, 847, 1293]
order: 107
---

# 状态压缩 BFS：搜索与图论训练题解

有些最短路题，光知道当前位置不够，还要知道已经拿了哪些钥匙、访问过哪些节点、还剩多少次消除障碍。这些都必须成为状态的一部分。

一句话记法：**BFS 的 visited 要标记完整状态，不只是位置。**

## 适用场景

- 最短路径访问所有节点。
- 网格中拿钥匙开锁。
- 可消除障碍的最短路。
- 状态维度较小，可以用 bitmask 表示集合。

## 状态设计

常见状态：

```text
(node, mask)        // 当前节点 + 已访问节点集合
(r, c, keys)        // 当前格子 + 已拿钥匙集合
(r, c, remain)      // 当前格子 + 剩余消除次数
```

如果只用 `(r,c)` 标记 visited，会错误剪掉“同一位置但钥匙更多”的更优状态。

## Go 参考骨架：访问所有节点

```go
func shortestPathLength(graph [][]int) int {
	n := len(graph)
	target := (1 << n) - 1
	type State struct{ node, mask int }
	q := []State{}
	dist := make([][]int, n)
	for i := range dist {
		dist[i] = make([]int, 1<<n)
		for j := range dist[i] {
			dist[i][j] = -1
		}
		mask := 1 << i
		dist[i][mask] = 0
		q = append(q, State{i, mask})
	}
	for head := 0; head < len(q); head++ {
		s := q[head]
		if s.mask == target {
			return dist[s.node][s.mask]
		}
		for _, v := range graph[s.node] {
			nm := s.mask | (1 << v)
			if dist[v][nm] == -1 {
				dist[v][nm] = dist[s.node][s.mask] + 1
				q = append(q, State{v, nm})
			}
		}
	}
	return -1
}
```

## 为什么这样写

在访问所有节点问题中，站在同一个节点但已访问集合不同，未来可达答案完全不同。所以 `visited[node]` 不够，必须是 `visited[node][mask]`。

多源初始化也很常见：可以从任意节点出发，就把所有 `(i, 1<<i)` 都作为 0 层状态入队。

## 复杂度

- 状态数通常是 $O(n \cdot 2^n)$ 或 $O(mnK)$。
- BFS 时间复杂度约为状态数乘转移数。

## 易错点

- visited 只按位置标记。
- mask 初始位没有包含当前节点。
- 多源 BFS 只放了一个起点。
- 状态空间过大还强行 BFS，需要重新建模。

## 练习顺序

建议按这个顺序刷：#847, #864, #1293。
