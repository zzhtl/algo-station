---
title: N 皇后列与斜线剪枝：回溯训练题解
category: 回溯
summary: N 皇后每一行只放一个皇后，递归按行推进；列、主对角线、副对角线三个集合决定当前位置能不能放。
problem_ids: [51, 52]
order: 108
---

# N 皇后列与斜线剪枝：回溯训练题解

N 皇后题的关键是建模：不用枚举棋盘上所有格子，而是按行递归。第 `row` 层只决定第 `row` 行的皇后放在哪一列。

一句话记法：**一行一个皇后，冲突只看列和两条斜线。**

## 适用场景

这种写法适合：

- 每一行或每一层必须放一个对象。
- 同层候选是列位置。
- 合法性由若干冲突集合决定。
- 需要输出所有棋盘，或只统计方案数量。

如果题目只问方案数，可以不构造字符串棋盘，只累加计数。

## 图解思路

```mermaid
flowchart TB
  A[row=0] --> B[枚举 col]
  B --> C{col / row-col / row+col 是否冲突?}
  C -->|是| B
  C -->|否| D[记录 queen[row]=col]
  D --> E[标记列和斜线]
  E --> F[row+1]
  F --> G[回溯撤销]
  G --> B
```

主对角线上的格子 `row - col` 相同，副对角线上的格子 `row + col` 相同。

## 不变量

- 当前正在处理第 `row` 行。
- `queens[r] = c` 表示第 `r` 行皇后放在第 `c` 列。
- `cols` 记录已经被占用的列。
- `diag1` 记录已经被占用的 `row - col`。
- `diag2` 记录已经被占用的 `row + col`。

只要这三个集合都不冲突，当前位置就可以放皇后。

## 手写步骤

1. 准备 `queens` 数组，长度为 `n`。
2. 准备列、主对角线、副对角线三个占用结构。
3. 定义 `dfs(row)`。
4. 如果 `row == n`，把 `queens` 转成棋盘。
5. 枚举当前行的每个 `col`。
6. 冲突则跳过；不冲突则放置、递归、撤销。

## Go 参考实现

```go
func solveNQueens(n int) [][]string {
	ans := [][]string{}
	queens := make([]int, n)
	for i := range queens {
		queens[i] = -1
	}
	cols := make([]bool, n)
	diag1 := make(map[int]bool)
	diag2 := make(map[int]bool)

	build := func() []string {
		board := make([]string, n)
		for r := 0; r < n; r++ {
			row := make([]byte, n)
			for c := 0; c < n; c++ {
				row[c] = '.'
			}
			row[queens[r]] = 'Q'
			board[r] = string(row)
		}
		return board
	}

	var dfs func(row int)
	dfs = func(row int) {
		if row == n {
			ans = append(ans, build())
			return
		}

		for col := 0; col < n; col++ {
			d1, d2 := row-col, row+col
			if cols[col] || diag1[d1] || diag2[d2] {
				continue
			}
			queens[row] = col
			cols[col], diag1[d1], diag2[d2] = true, true, true
			dfs(row + 1)
			cols[col], diag1[d1], diag2[d2] = false, false, false
			queens[row] = -1
		}
	}

	dfs(0)
	return ans
}
```

## Rust 参考实现

```rust
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn build(queens: &[usize], n: usize) -> Vec<String> {
        let mut board = Vec::with_capacity(n);
        for &col in queens {
            let mut row = vec![b'.'; n];
            row[col] = b'Q';
            board.push(String::from_utf8(row).unwrap());
        }
        board
    }

    fn dfs(
        row: usize,
        n: usize,
        queens: &mut Vec<usize>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            ans.push(build(queens, n));
            return;
        }

        for col in 0..n {
            let d1 = row + n - 1 - col;
            let d2 = row + col;
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }
            queens.push(col);
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;
            dfs(row + 1, n, queens, cols, diag1, diag2, ans);
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
            queens.pop();
        }
    }

    let n = n as usize;
    let mut ans = Vec::new();
    let mut queens = Vec::new();
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1];
    let mut diag2 = vec![false; 2 * n - 1];
    dfs(0, n, &mut queens, &mut cols, &mut diag1, &mut diag2, &mut ans);
    ans
}
```

## 为什么这样写

按行递归隐含了一个强约束：前 `row` 行都已经各放了一个皇后，所以当前只需要找本行合法列。这样比“从全棋盘选 n 个格子”小很多，也更容易维护状态。

斜线判断来自坐标性质：

- 同一条主对角线，`row - col` 相等。
- 同一条副对角线，`row + col` 相等。

Rust 里 `row - col` 可能为负，所以上面用 `row + n - 1 - col` 做偏移，把范围变成 `0..2n-1`。

## 复杂度

- 搜索复杂度上界接近 $O(n!)$，剪枝会显著减少实际分支。
- 构造每个棋盘需要 $O(n^2)$。
- 不计输出，递归深度和状态数组是 $O(n)$ 到 $O(n^2)$ 之间，取决于是否构造棋盘。

## 易错点

- 逐格递归，导致状态复杂且分支过多。
- 只检查列，忘记检查两条斜线。
- Rust 中直接用 `row - col` 作为 `usize` 下标导致下溢。
- 生成棋盘时复用同一行缓冲区，导致答案被覆盖。

## 练习顺序

建议按这个顺序刷：#51, #52。

#51 练完整构造棋盘，#52 只统计数量。两题核心完全一样，差别只在终止条件处如何记录答案。
