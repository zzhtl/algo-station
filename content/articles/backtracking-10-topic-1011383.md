---
title: 单词搜索原地标记：回溯训练题解
category: 回溯
summary: 单词搜索是网格 DFS：当前位置必须匹配当前字符，向四个方向递归前要临时标记已访问，返回时恢复。
problem_ids: [79, 212]
order: 110
---

# 单词搜索原地标记：回溯训练题解

单词搜索的搜索状态不是 `path` 里的数组，而是网格坐标和当前匹配到的字符下标。每个格子在同一条路径中只能使用一次，所以需要访问标记。

一句话记法：**先匹配当前格，再标记访问，四向递归，最后恢复。**

## 适用场景

适合这种写法的题：

- 在二维网格中寻找路径。
- 每一步只能移动到相邻格子。
- 同一条路径不能重复使用同一个格子。
- 搜索过程中可以原地标记，并在回溯时恢复。

如果要同时搜索大量单词，比如 #212，单纯对每个单词跑一次 DFS 会偏慢，通常要结合 Trie 剪枝。

## 图解思路

```mermaid
flowchart TB
  A[选择起点] --> B{board[r][c] 是否等于 word[k]?}
  B -->|否| X[失败]
  B -->|是| C{是否匹配到最后一个字符?}
  C -->|是| Y[成功]
  C -->|否| D[标记当前格已访问]
  D --> E[向上下左右递归 k+1]
  E --> F[恢复当前格]
```

注意恢复必须发生在所有方向尝试之后，否则兄弟路径会看到错误的棋盘状态。

## 不变量

- `dfs(r, c, k)` 表示尝试用 `board[r][c]` 匹配 `word[k]`。
- 进入四向递归前，当前格已经被标记为不可再用。
- 当前递归返回前，必须把当前格恢复成原字符。
- 如果 `k == word.len() - 1` 且当前格匹配，说明整词找到。

## 手写步骤

1. 枚举每个格子作为起点。
2. 在 `dfs(r, c, k)` 中先检查越界和字符是否匹配。
3. 如果已经匹配最后一个字符，返回 `true`。
4. 保存当前字符，原地改成特殊标记。
5. 向四个方向递归匹配 `k + 1`。
6. 恢复当前字符。
7. 任何方向成功就返回 `true`。

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

## Rust 参考实现

```rust
pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    let m = board.len();
    let n = board[0].len();

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], r: i32, c: i32, k: usize) -> bool {
        if r < 0 || c < 0 {
            return false;
        }
        let (r, c) = (r as usize, c as usize);
        if r >= board.len() || c >= board[0].len() || board[r][c] != word[k] {
            return false;
        }
        if k + 1 == word.len() {
            return true;
        }

        let ch = board[r][c];
        board[r][c] = '#';
        let found = dfs(board, word, r as i32 + 1, c as i32, k + 1)
            || dfs(board, word, r as i32 - 1, c as i32, k + 1)
            || dfs(board, word, r as i32, c as i32 + 1, k + 1)
            || dfs(board, word, r as i32, c as i32 - 1, k + 1);
        board[r][c] = ch;
        found
    }

    for r in 0..m {
        for c in 0..n {
            if dfs(&mut board, &word, r as i32, c as i32, 0) {
                return true;
            }
        }
    }
    false
}
```

## 为什么这样写

网格回溯的状态在棋盘上，访问标记就是路径的一部分。原地标记能少维护一个 `visited` 矩阵，但要求更严格：任何返回路径都必须恢复原字符，包括提前找到答案的路径。

递归函数先判断当前格是否匹配 `word[k]`，而不是先移动再判断，可以让含义更统一：每一层都负责消费一个字符。

## 复杂度

- 设网格大小为 `m*n`，单词长度为 `L`。
- 每个起点最多向外搜索，首步之后通常最多 3 个方向，粗略上界是 $O(mn \cdot 4 \cdot 3^{L-1})$。
- 不计棋盘本身，递归深度是 $O(L)$。

## 易错点

- 没有标记访问，导致同一格在一条路径中被重复使用。
- 成功提前返回前忘记恢复当前格。
- 把 `k == word.len()` 和当前字符匹配顺序写乱，出现越界。
- Rust 中用 `usize` 做坐标直接减一，导致下溢；可以用 `i32` 坐标处理边界。

## 练习顺序

建议按这个顺序刷：#79, #212。

#79 练单词级 DFS 和原地恢复；#212 再引入 Trie，把“当前前缀是否存在”也变成剪枝条件。
