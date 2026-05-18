---
title: 中序验证搜索树：二叉树训练题解
category: 二叉树
summary: 二叉搜索树的中序遍历必须严格递增；验证 BST 可以维护上一个访问值，发现非递增立即失败。
problem_ids: [98, 230, 700]
order: 105
---

# 中序验证搜索树：二叉树训练题解

BST 的核心性质不是“左孩子小、右孩子大”这么简单，而是左子树所有节点都小于当前节点，右子树所有节点都大于当前节点。中序遍历正好能把 BST 输出成严格递增序列。

一句话记法：**中序遍历 BST，访问值必须严格递增。**

## 适用场景

- 验证二叉搜索树。
- BST 第 k 小。
- BST 中序迭代器。
- 需要利用 BST 有序性质的题。

如果只比较直接左右孩子，会漏掉深层节点违反范围的情况。

## Go 参考实现：验证 BST

```go
func isValidBST(root *TreeNode) bool {
	var prev *TreeNode
	var dfs func(*TreeNode) bool
	dfs = func(node *TreeNode) bool {
		if node == nil {
			return true
		}
		if !dfs(node.Left) {
			return false
		}
		if prev != nil && prev.Val >= node.Val {
			return false
		}
		prev = node
		return dfs(node.Right)
	}
	return dfs(root)
}
```

## 另一种写法：范围约束

```go
func isValidBSTRange(root *TreeNode) bool {
	var dfs func(*TreeNode, int64, int64) bool
	dfs = func(node *TreeNode, low, high int64) bool {
		if node == nil {
			return true
		}
		v := int64(node.Val)
		if v <= low || v >= high {
			return false
		}
		return dfs(node.Left, low, v) && dfs(node.Right, v, high)
	}
	return dfs(root, math.MinInt64, math.MaxInt64)
}
```

## 为什么这样写

中序验证利用了 BST 的全局有序性。只要遍历过程中出现 `prev >= current`，说明某个节点放错了位置。

范围约束写法则把祖先限制传下去：左子树上界变成当前值，右子树下界变成当前值。它能直接表达“整棵左子树都要小于当前节点”。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：递归栈 $O(h)$。

## 易错点

- 只判断 `node.Left.Val < node.Val < node.Right.Val`。
- BST 要严格递增，重复值通常不合法。
- 使用 `int` 边界时遇到 `MinInt/MaxInt` 溢出，范围法建议用更宽类型。
- 中序遍历中 `prev` 更新位置写错。

## 练习顺序

建议按这个顺序刷：#98, #230, #700。
