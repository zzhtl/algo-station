---
title: 回溯搜索单词路径：搜索与图论训练题解
category: 搜索与图论
summary: 网格单词搜索是带访问标记的 DFS：当前格匹配当前字符后，临时标记已用，再向四邻递归。
problem_ids: [79, 212]
order: 105
---

# 回溯搜索单词路径：搜索与图论训练题解

单词搜索不是最短路，而是枚举一条不能重复使用格子的路径。每次尝试一个方向后，都要恢复当前格。

一句话记法：**匹配、标记、递归、恢复。**

## Go 参考实现

```go
func exist(board [][]byte, word string) bool {
	m, n := len(board), len(board[0])
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	var dfs func(int, int, int) bool
	dfs = func(r, c, k int) bool {
		if r < 0 || r >= m || c < 0 || c >= n || board[r][c] != word[k] {
			return false
		}
		if k == len(word)-1 {
			return true
		}
		ch := board[r][c]
		board[r][c] = '#'
		for _, d := range dirs {
			if dfs(r+d[0], c+d[1], k+1) {
				board[r][c] = ch
				return true
			}
		}
		board[r][c] = ch
		return false
	}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if dfs(r, c, 0) {
				return true
			}
		}
	}
	return false
}
```

## 为什么这样写

同一条路径里一个格子只能用一次，所以当前格进入下一层前必须标记。递归返回后恢复，是为了让其他起点或其他路径还能使用这个格子。

#212 多单词搜索时，逐个单词跑 DFS 会重复大量前缀搜索，通常要用 Trie 把前缀剪枝也加进去。

## 复杂度

- 单词长度为 `L` 时，粗略上界 $O(mn \cdot 4 \cdot 3^{L-1})$。
- 递归深度 $O(L)$。

## 易错点

- 不标记访问，同一格被重复使用。
- 成功提前返回前忘记恢复。
- `k` 到末尾和字符匹配顺序写乱。
- 多单词版本没有 Trie，复杂度过高。

## 练习顺序

建议按这个顺序刷：#79, #212。
