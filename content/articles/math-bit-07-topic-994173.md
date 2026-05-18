---
title: 进制转换逐位处理：数学与位运算训练题解
category: 数学与位运算
summary: 用取模拿低位、整除推进高位，训练整数反转、加一和二进制加法的统一写法。
problem_ids: [7, 66, 67]
order: 107
---

# 进制转换逐位处理：数学与位运算训练题解

很多数字题都可以看成逐位处理：十进制取个位用 `%10`，去掉个位用 `/10`；二进制或字符串加法则从末尾向前处理，并维护进位。

关键不是背函数，而是把每一位的局部规则写清楚。

## 整数反转

反转整数时，每次取出最低位 `digit`，把它接到答案末尾：

```text
ans = ans * 10 + digit
```

需要在更新前检查是否会溢出。

```go
package main

import "math"

func reverse(x int) int {
	ans := 0
	for x != 0 {
		digit := x % 10
		x /= 10

		if ans > math.MaxInt32/10 || ans < math.MinInt32/10 {
			return 0
		}
		ans = ans*10 + digit
	}
	return ans
}
```

## 二进制字符串加法

从右往左处理，局部状态是 `carry`：

```go
func addBinary(a string, b string) string {
	i, j := len(a)-1, len(b)-1
	carry := 0
	ans := []byte{}

	for i >= 0 || j >= 0 || carry > 0 {
		sum := carry
		if i >= 0 {
			sum += int(a[i] - '0')
			i--
		}
		if j >= 0 {
			sum += int(b[j] - '0')
			j--
		}
		ans = append(ans, byte('0'+sum%2))
		carry = sum / 2
	}

	for l, r := 0, len(ans)-1; l < r; l, r = l+1, r-1 {
		ans[l], ans[r] = ans[r], ans[l]
	}
	return string(ans)
}
```

## 统一思路

逐位题一般都有三个问题：

- 当前位怎么取：数字用 `% base`，字符串用下标。
- 剩余部分怎么推进：数字用 `/ base`，字符串移动指针。
- 进位或借位怎么保存：用 `carry` 或 `borrow`。

把这三点写清，代码就不会乱。

## 易错点

- 整数反转要在乘 10 前做溢出检查。
- 字符串加法最后要反转答案。
- 循环条件要包含 `carry > 0`，否则最后一位进位会丢。

## 复杂度

逐位处理的时间与位数成正比，空间取决于是否需要构造新结果。
