---
title: 快乐数快慢指针：数学与位运算训练题解
category: 数学与位运算
summary: 把数字平方和看成状态转移，用环检测判断最终到 1 还是进入循环。
problem_ids: [202]
order: 110
---

# 快乐数快慢指针：数学与位运算训练题解

快乐数的过程是：不断把数字替换成“各位数字平方和”。如果最终变成 `1`，就是快乐数；如果不是，就会进入循环。

这其实是一个函数图：

```text
n -> next(n) -> next(next(n)) -> ...
```

每个状态只有一个后继。判断是否到 `1`，可以用哈希集合记录访问过的数，也可以用快慢指针检测环。

## Go 参考实现

```go
package main

func nextHappy(n int) int {
	sum := 0
	for n > 0 {
		digit := n % 10
		sum += digit * digit
		n /= 10
	}
	return sum
}

func isHappy(n int) bool {
	slow, fast := n, nextHappy(n)
	for fast != 1 && slow != fast {
		slow = nextHappy(slow)
		fast = nextHappy(nextHappy(fast))
	}
	return fast == 1
}
```

## 为什么一定会重复

位数很大的数经过一次平方和后会急剧变小。例如 32 位整数最多 10 位，每位最大贡献 `81`，一次后不超过 `810`。后续状态空间有限，所以不是到 `1`，就是进入某个环。

快慢指针不需要保存所有状态，只要存在环，快指针最终会追上慢指针。

## 易错点

- `nextHappy` 处理的是十进制位，不是二进制位。
- 循环条件里要同时判断 `fast != 1` 和 `slow != fast`。
- 如果用哈希集合写法，发现重复时返回 `false`，不是继续循环。

## 复杂度

单次 `nextHappy` 与数字位数相关。状态会很快收敛到小范围，通常可视为 `O(log n)` 时间，空间 `O(1)`。
