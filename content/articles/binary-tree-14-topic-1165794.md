---
title: 二叉搜索树范围剪枝：二叉树训练题解
category: 二叉树
summary: BST 题要主动利用范围：当前值太小就去右子树，太大就去左子树，只有落在范围内才同时处理两边。
problem_ids: [700, 701, 450, 938, 98]
order: 114
---

# 二叉搜索树范围剪枝：二叉树训练题解

BST 的价值在于有序。很多题不需要遍历整棵树，只要根据当前节点值和目标范围决定走哪边。

一句话记法：**小了去右，大了去左，命中范围才两边都看。**

## 适用场景

- BST 搜索。
- BST 插入和删除。
- BST 范围和。
- 验证 BST 的范围递归。

不要把 BST 当普通二叉树全量 DFS，能剪枝就剪。

## Go 参考实现：范围和

```go
func rangeSumBST(root *TreeNode, low int, high int) int {
	if root == nil {
		return 0
	}
	if root.Val < low {
		return rangeSumBST(root.Right, low, high)
	}
	if root.Val > high {
		return rangeSumBST(root.Left, low, high)
	}
	return root.Val +
		rangeSumBST(root.Left, low, high) +
		rangeSumBST(root.Right, low, high)
}
```

## 搜索节点

```go
func searchBST(root *TreeNode, val int) *TreeNode {
	for root != nil {
		if root.Val == val {
			return root
		}
		if val < root.Val {
			root = root.Left
		} else {
			root = root.Right
		}
	}
	return nil
}
```

## 为什么这样写

如果当前节点值小于 `low`，那么它的左子树所有值都更小，也一定小于 `low`，可以整棵剪掉，只去右子树。大于 `high` 时同理剪掉右子树。

这就是 BST 范围剪枝的核心：不是“遍历后判断要不要加”，而是用当前值排除整棵不可能贡献答案的子树。

## 复杂度

- 搜索平均 $O(h)$，平衡树是 $O(\log n)$，最坏退化 $O(n)$。
- 范围和复杂度与访问到的节点数有关，最坏 $O(n)$。
- 空间复杂度：递归栈 $O(h)$。

## 易错点

- 当前值小于 low 时还去左子树，浪费且可能误解性质。
- 把 BST 题当普通树遍历，没利用剪枝。
- 删除 BST 节点时没有保持中序有序性。
- 验证 BST 时只比较直接孩子，没传上下界。

## 练习顺序

建议按这个顺序刷：#700, #938, #701, #450, #98。
