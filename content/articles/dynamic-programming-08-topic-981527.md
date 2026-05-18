---
title: 记忆化搜索剪重复：动态规划训练题解
category: 动态规划
summary: 记忆化搜索保留递归思路，但把重复子问题缓存起来；适合先写暴力递归再优化的 DP 题。
problem_ids: [322, 139, 300, 329]
order: 108
---

# 记忆化搜索剪重复：动态规划训练题解

动态规划不一定一开始就写表。有些题先写递归更自然，只要发现同一个状态被反复计算，就可以加 memo 变成记忆化搜索。

一句话记法：**递归定义状态，memo 避免重复。**

## 适用场景

- 自底向上填表顺序不直观。
- 状态图是 DAG，没有无限递归。
- 暴力递归里有大量重复子问题。
- 只会访问部分状态，用记忆化更省。

如果状态依赖有环，要先处理环或改建模。

## 手写步骤

1. 写递归函数 `dfs(state)`，明确返回值含义。
2. 写边界条件。
3. 写递归转移。
4. 用 map/数组缓存 `state -> answer`。
5. 每次进入先查 memo，算完再存。

## Go 参考实现：零钱兑换

```go
func coinChange(coins []int, amount int) int {
	memo := map[int]int{}
	const inf = int(1e9)
	var dfs func(int) int
	dfs = func(rest int) int {
		if rest == 0 {
			return 0
		}
		if rest < 0 {
			return inf
		}
		if v, ok := memo[rest]; ok {
			return v
		}
		best := inf
		for _, c := range coins {
			if sub := dfs(rest - c); sub+1 < best {
				best = sub + 1
			}
		}
		memo[rest] = best
		return best
	}
	ans := dfs(amount)
	if ans >= inf {
		return -1
	}
	return ans
}
```

## 为什么这样写

递归 `dfs(rest)` 表示凑出金额 `rest` 的最少硬币数。它会调用 `dfs(rest-c)`。不同路径可能到达同一个 `rest`，这就是重复子问题。

memo 的 key 必须完整描述状态。如果状态里有两个变量，比如区间 `[l,r]`，key 就不能只存 `l`。

## 复杂度

- 时间复杂度约等于“状态数 × 每个状态转移成本”。
- 空间复杂度是 memo 状态数加递归栈。

## 易错点

- memo key 没包含完整状态。
- 失败状态没有缓存，导致仍然指数级。
- 递归有环，memo 还没写入就再次访问同一状态。
- `inf + 1` 溢出，工程里要选安全哨兵。

## 练习顺序

建议按这个顺序刷：#322, #139, #329。

先从一维状态开始，再练字符串位置和网格位置状态。
