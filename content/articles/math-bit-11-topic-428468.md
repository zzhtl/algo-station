---
title: 组合数递推：数学与位运算训练题解
category: 数学与位运算
summary: 用上一项推出下一项，避免阶乘溢出并保持组合数计算过程可控。
problem_ids: [118, 119, 62]
order: 111
---

# 组合数递推：数学与位运算训练题解

组合数常见公式是：

```text
C(n,k) = n! / (k!(n-k)!)
```

但直接算阶乘容易溢出，也做了很多不必要的乘法。更稳定的做法是用相邻项递推：

```text
C(n,0) = 1
C(n,i) = C(n,i-1) * (n-i+1) / i
```

## 求杨辉三角某一行

```go
package main

func getRow(rowIndex int) []int {
	row := make([]int, rowIndex+1)
	row[0] = 1

	for i := 1; i <= rowIndex; i++ {
		row[i] = int(int64(row[i-1]) * int64(rowIndex-i+1) / int64(i))
	}
	return row
}
```

递推过程中每一步都能整除，因为组合数本身是整数。中间乘法用 `int64` 更稳。

## 用在路径计数

从左上角走到右下角，如果只能向右和向下，`m x n` 网格一共要走 `m+n-2` 步，其中选择 `m-1` 步向下：

```go
func uniquePaths(m int, n int) int {
	total := m + n - 2
	choose := m - 1
	if n-1 < choose {
		choose = n - 1
	}

	ans := int64(1)
	for i := 1; i <= choose; i++ {
		ans = ans * int64(total-choose+i) / int64(i)
	}
	return int(ans)
}
```

选择较小的 `k` 计算，可以减少循环次数和中间数规模。

## 易错点

- 不要先算阶乘再相除，溢出风险高。
- 递推式中的乘除顺序建议先乘再除，并用更大整数类型承接。
- `C(n,k) == C(n,n-k)`，先把 `k` 化成较小的一边。

## 复杂度

计算单个组合数时间 `O(k)`，空间 `O(1)`；生成一行时间 `O(n)`，空间 `O(n)`。
