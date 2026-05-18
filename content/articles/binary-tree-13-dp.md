---
title: 树形 DP 选与不选：二叉树训练题解
category: 二叉树
summary: 树形 DP 常让每个节点返回两个状态：选当前节点的最优值、不选当前节点的最优值，父节点再合并。
problem_ids: [337]
order: 113
---

# 树形 DP 选与不选：二叉树训练题解

树形 DP 的特点是：每个节点的选择会影响孩子能不能选。最典型的是打家劫舍 III，选了当前节点，就不能选它的左右孩子。

一句话记法：**每个节点返回两个值：选它、不选它。**

## 适用场景

- 树上相邻节点不能同时选择。
- 每个节点需要向父节点返回多种状态。
- 当前节点答案由左右子树状态合并而来。

这类题不要只返回一个最大值，否则父节点不知道你到底选没选当前节点。

## Go 参考实现

```go
func rob(root *TreeNode) int {
	var dfs func(*TreeNode) (int, int)
	dfs = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}
		leftRob, leftSkip := dfs(node.Left)
		rightRob, rightSkip := dfs(node.Right)

		robThis := node.Val + leftSkip + rightSkip
		skipThis := max(leftRob, leftSkip) + max(rightRob, rightSkip)
		return robThis, skipThis
	}
	robRoot, skipRoot := dfs(root)
	return max(robRoot, skipRoot)
}
```

## 为什么这样写

如果选当前节点，左右孩子都不能选，所以收益是：

```text
node.Val + leftSkip + rightSkip
```

如果不选当前节点，左右孩子各自可以选或不选，取各自最大值：

```text
max(leftRob, leftSkip) + max(rightRob, rightSkip)
```

父节点需要这两个状态，所以当前节点必须同时返回。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：递归栈 $O(h)$。

## 易错点

- 只返回一个最大值，父节点无法判断孩子是否被选。
- 选当前节点时又加了孩子的最大值，违反相邻限制。
- 不选当前节点时误以为必须选孩子。
- 用记忆化写法但以节点值作为 key，重复值会冲突。

## 练习顺序

建议先刷 #337。

复盘时重点不是“偷不偷”，而是状态设计：父节点需要知道子节点哪几种情况。
