---
title: 状态机 DP 分阶段：动态规划训练题解
category: 动态规划
summary: 状态机 DP 把“当前处于什么阶段”写进状态，例如股票题里的持有/不持有、冷冻期、交易次数。
problem_ids: [121, 122, 123, 188, 309, 714]
order: 107
---

# 状态机 DP 分阶段：动态规划训练题解

有些 DP 不能只用一个下标表示进度，还要知道当前处于什么状态。股票买卖就是典型：今天结束后你是持有股票，还是不持有股票？

一句话记法：**状态表示阶段，转移表示动作。**

## 适用场景

- 股票买卖系列。
- 有冷冻期、手续费、交易次数限制。
- 每一步有明确动作导致状态切换。
- 需要避免非法动作，比如没股票时不能卖。

## 基础状态

```text
hold = 今天结束后持有股票的最大收益
cash = 今天结束后不持有股票的最大收益
```

每天有两类动作：

- 买入：`cash - price`
- 卖出：`hold + price`

## Go 参考实现：含手续费

```go
func maxProfit(prices []int, fee int) int {
	hold := -prices[0]
	cash := 0
	for i := 1; i < len(prices); i++ {
		price := prices[i]
		newCash := cash
		if hold+price-fee > newCash {
			newCash = hold + price - fee
		}
		newHold := hold
		if cash-price > newHold {
			newHold = cash - price
		}
		cash, hold = newCash, newHold
	}
	return cash
}
```

## 为什么这样写

`cash` 的来源有两个：昨天就不持有，或者今天把股票卖掉。`hold` 的来源也有两个：昨天就持有，或者今天买入。

手续费可以放在买入时扣，也可以放在卖出时扣，只要全程一致。最后答案必须是不持有状态，因为持有股票还没兑现收益。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：基础版 $O(1)$；带交易次数限制通常是 $O(k)$。

## 易错点

- 同一天更新顺序污染旧状态，复杂版本建议先算新变量。
- 最后返回 `hold`。
- 手续费买卖两边都扣。
- 交易次数限制没有写进状态。

## 练习顺序

建议按这个顺序刷：#121, #122, #714, #309, #123, #188。
