---
title: 二维前缀和封装查询：设计与数据结构训练题解
category: 设计与数据结构
summary: 用多一行一列的二维前缀和统一矩形查询，避免边界分支。
problem_ids: [304]
order: 107
---

# 二维前缀和封装查询：设计与数据结构训练题解

二维不可变矩阵的区域和查询，和一维前缀和一样，适合把计算放到构造阶段。多开一行一列后，任意矩形都能用四个前缀值算出来。

## 状态定义

`prefix[i][j]` 表示原矩阵中左上角 `(0,0)` 到 `(i-1,j-1)` 这块区域的和，不包含第 `i` 行和第 `j` 列。

构造公式：

```text
prefix[i+1][j+1] = matrix[i][j]
                 + prefix[i][j+1]
                 + prefix[i+1][j]
                 - prefix[i][j]
```

查询 `(r1,c1)` 到 `(r2,c2)`：

```text
sum = prefix[r2+1][c2+1]
    - prefix[r1][c2+1]
    - prefix[r2+1][c1]
    + prefix[r1][c1]
```

## Go 参考实现

```go
package main

type NumMatrix struct {
	prefix [][]int
}

func Constructor(matrix [][]int) NumMatrix {
	m := len(matrix)
	n := 0
	if m > 0 {
		n = len(matrix[0])
	}
	prefix := make([][]int, m+1)
	for i := range prefix {
		prefix[i] = make([]int, n+1)
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			prefix[i+1][j+1] = matrix[i][j] + prefix[i][j+1] + prefix[i+1][j] - prefix[i][j]
		}
	}
	return NumMatrix{prefix: prefix}
}

func (nm *NumMatrix) SumRegion(row1 int, col1 int, row2 int, col2 int) int {
	p := nm.prefix
	return p[row2+1][col2+1] - p[row1][col2+1] - p[row2+1][col1] + p[row1][col1]
}
```

## 为什么要加回左上角

查询时减掉上方区域和左方区域，这两块都包含左上角重叠区域，所以重叠区域被减了两次，需要加回来一次。

二维前缀和的易错点几乎都在下标，使用多一行一列可以让所有公式都不需要特殊判断边界。

## 易错点

- `prefix` 的维度是 `(m+1) x (n+1)`。
- 查询右下角要用 `row2+1`、`col2+1`。
- 空矩阵要能构造出合法对象，避免访问 `matrix[0]` 崩溃。

## 复杂度

构造时间 `O(mn)`，查询时间 `O(1)`，空间 `O(mn)`。
