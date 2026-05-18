---
title: 树形 DP 汇总子树：动态规划训练题解
category: 动态规划
summary: 树形 DP 通常在后序位置合并左右子树结果；每个节点返回父节点需要的状态，而不是整棵树的全部答案。
problem_ids: [337, 124, 543, 968]
order: 106
---

# 树形 DP 汇总子树：动态规划训练题解

树形 DP 是把 DP 状态放到树节点上。当前节点的状态通常来自左右孩子，所以天然适合后序递归。

一句话记法：**子树先给答案，当前节点再合并状态。**

## 适用场景

- 树上选与不选。
- 最大路径和、直径。
- 二叉树监控。
- 当前节点状态依赖孩子状态。

树形 DP 最重要的是定义“返回给父节点的是什么”。

## Go 参考实现：打家劫舍 III

```go
func rob(root *TreeNode) int {
	var dfs func(*TreeNode) (int, int)
	dfs = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}
		lRob, lSkip := dfs(node.Left)
		rRob, rSkip := dfs(node.Right)
		robThis := node.Val + lSkip + rSkip
		skipThis := max(lRob, lSkip) + max(rRob, rSkip)
		return robThis, skipThis
	}
	a, b := dfs(root)
	return max(a, b)
}
```

## 为什么这样写

树上没有线性下标，状态依附在节点上。对于每个节点，父节点真正关心的是：如果我选你，收益是多少；如果我不选你，收益是多少。

所以返回两个状态比返回一个最大值更完整。父节点再根据自己的选择决定使用哪一个。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：递归栈 $O(h)$。

## 易错点

- 当前节点依赖孩子状态，却在前序位置计算。
- 返回值含义不清，父节点无法正确合并。
- 用全局变量和返回值混在一起。
- 树上相邻约束题只返回一个最大值。

## 练习顺序

建议按这个顺序刷：#337, #543, #124, #968。
