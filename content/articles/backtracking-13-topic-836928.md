---
title: 图路径回溯防环：回溯训练题解
category: 回溯
summary: 图路径回溯的状态是当前节点和路径；有环图必须维护访问状态，DAG 则可以省掉防环但仍要正确撤销路径。
problem_ids: [797, 332, 841]
order: 113
---

# 图路径回溯防环：回溯训练题解

图路径题和数组回溯不同：下一步候选不是下标范围，而是当前节点的邻居。递归每深入一层，就把路径延长到一个新节点。

一句话记法：**当前节点决定下一批候选；有环就标记访问，返回时撤销。**

## 适用场景

适合这种写法的题：

- 要枚举从起点到终点的所有路径。
- 路径上的节点或边不能重复使用。
- 图可能有环，需要避免递归绕圈。
- 候选集合来自邻接表。

如果只判断能否到达，普通 DFS/BFS 就够了；只有需要记录路径或枚举方案时，才体现回溯。

## 图解思路

```mermaid
flowchart TB
  A[当前节点 u] --> B[遍历 graph[u] 的邻居 v]
  B --> C{v 是否可进入?}
  C -->|否| B
  C -->|是| D[把 v 加入 path 并标记]
  D --> E[递归到 v]
  E --> F[撤销 v 和访问标记]
  F --> B
```

对 DAG 的所有路径题，图本身无环，可以不使用 `visited`；对一般图，必须根据题意维护节点或边的访问状态。

## 不变量

- `path` 是从起点到当前节点的真实路径。
- `path.last()` 等于当前节点。
- 如果图可能有环，`visited[x]` 表示节点 `x` 是否已经在当前路径中。
- 到达终点时，复制当前路径。

注意 `visited` 表示“当前路径里访问过”，不是“全局已经访问过”。枚举所有路径时，不能因为一个节点在别的路径里出现过，就永远不让它再出现。

## 手写步骤

1. 建邻接表。
2. 把起点放入 `path`，必要时标记起点访问。
3. 定义 `dfs(u)`。
4. 如果 `u` 是终点，复制路径。
5. 遍历 `graph[u]` 的所有邻居 `v`。
6. 如果 `v` 当前路径中已经访问过，跳过。
7. 加入路径并标记，递归，返回后撤销。

## Go 参考实现：DAG 所有路径

```go
func allPathsSourceTarget(graph [][]int) [][]int {
	target := len(graph) - 1
	ans := [][]int{}
	path := []int{0}

	var dfs func(u int)
	dfs = func(u int) {
		if u == target {
			ans = append(ans, append([]int(nil), path...))
			return
		}

		for _, v := range graph[u] {
			path = append(path, v)
			dfs(v)
			path = path[:len(path)-1]
		}
	}

	dfs(0)
	return ans
}
```

## Rust 参考实现：一般图防环骨架

```rust
pub fn enumerate_paths(graph: Vec<Vec<usize>>, start: usize, target: usize) -> Vec<Vec<usize>> {
    fn dfs(
        u: usize,
        target: usize,
        graph: &[Vec<usize>],
        visited: &mut [bool],
        path: &mut Vec<usize>,
        ans: &mut Vec<Vec<usize>>,
    ) {
        if u == target {
            ans.push(path.clone());
            return;
        }

        for &v in &graph[u] {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            path.push(v);
            dfs(v, target, graph, visited, path, ans);
            path.pop();
            visited[v] = false;
        }
    }

    let mut visited = vec![false; graph.len()];
    let mut path = vec![start];
    let mut ans = Vec::new();
    visited[start] = true;
    dfs(start, target, &graph, &mut visited, &mut path, &mut ans);
    ans
}
```

## 为什么这样写

在图上枚举路径时，不能照搬数组回溯的 `start`。`start` 的意义是“下标只往后走”，而图的下一步由边决定。

DAG 的 #797 不需要 `visited`，因为题目保证没有环；路径不可能绕回之前节点。但如果换成一般图，必须使用当前路径级别的访问标记，否则递归可能无限循环。

同时也不能把 `visited` 当成全局剪枝。枚举所有路径时，一个节点可以出现在多条不同路径里，只是不能在同一条路径中重复出现。

## 复杂度

- 枚举所有路径的复杂度由路径数量决定，最坏可能指数级。
- 每条路径复制成本与路径长度有关。
- 不计输出，递归深度最多是节点数，`visited` 是 $O(V)$。

## 易错点

- 一般图没有防环，导致无限递归。
- 把 `visited` 写成全局访问后不撤销，漏掉经过同一节点的其他合法路径。
- 到达终点时直接保存 `path` 引用，没有复制。
- 把邻接表候选误写成数组下标范围。

## 练习顺序

建议按这个顺序刷：#797, #841, #332。

#797 先练 DAG 路径枚举；#841 练图可达性，体会不需要路径时可以简化；#332 再看“边只能用一次”的变体，访问状态要从节点转成边或票。
