---
title: 直径在后序中更新：二叉树训练题解
category: 二叉树
summary: 二叉树直径可能穿过当前节点，更新答案要用左右高度相加；但返回父节点时只能返回单边高度。
problem_ids: [543, 124]
order: 107
---

# 直径在后序中更新：二叉树训练题解

二叉树直径是任意两个节点之间最长路径。它可能完全在左子树、完全在右子树，也可能穿过当前节点。后序递归能同时处理这三种情况。

一句话记法：**更新答案可以左右都用，返回父节点只能选一边。**

## 适用场景

- 二叉树直径。
- 二叉树最大路径和。
- 路径可以在当前节点分叉，但向父节点返回时不能分叉。

这是树题中“全局答案”和“返回值”必须分清的代表模型。

## Go 参考实现：二叉树直径

```go
func diameterOfBinaryTree(root *TreeNode) int {
	ans := 0
	var depth func(*TreeNode) int
	depth = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left := depth(node.Left)
		right := depth(node.Right)
		if left+right > ans {
			ans = left + right
		}
		if left > right {
			return left + 1
		}
		return right + 1
	}
	depth(root)
	return ans
}
```

## 最大路径和的同构点

最大路径和也一样：更新答案时可以用 `left + node.Val + right`，返回父节点时只能返回 `node.Val + max(left, right)`。如果子树贡献为负，要当成 0。

## 为什么这样写

一条向父节点延伸的路径，不能同时从当前节点走向左右两边，否则到父节点后会变成叉形，不再是一条简单路径。

但全局答案可以在当前节点结算“左边最深路径 + 当前节点 + 右边最深路径”。所以更新答案和返回值不是同一个东西。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：递归栈 $O(h)$。

## 易错点

- 返回 `left + right + 1` 给父节点，导致路径分叉。
- 直径按节点数还是边数没分清；#543 返回边数，所以用 `left + right`。
- 最大路径和中没有丢弃负贡献。
- 只返回高度，没有更新全局答案。

## 练习顺序

建议按这个顺序刷：#543, #124。
