---
title: 镜像递归比较：二叉树训练题解
category: 二叉树
summary: 对称二叉树不是比较左右子树是否相同，而是比较外侧和内侧是否互为镜像。
problem_ids: [101, 100, 226]
order: 111
---

# 镜像递归比较：二叉树训练题解

对称二叉树要比较的是两棵子树是否镜像：左子树的左边要对右子树的右边，左子树的右边要对右子树的左边。

一句话记法：**外侧对外侧，内侧对内侧。**

## 适用场景

- 判断二叉树是否对称。
- 判断两棵树是否相同。
- 翻转二叉树后比较结构。

镜像比较和相同树比较只差递归配对方式。

## Go 参考实现：对称二叉树

```go
func isSymmetric(root *TreeNode) bool {
	var check func(*TreeNode, *TreeNode) bool
	check = func(a, b *TreeNode) bool {
		if a == nil && b == nil {
			return true
		}
		if a == nil || b == nil {
			return false
		}
		if a.Val != b.Val {
			return false
		}
		return check(a.Left, b.Right) && check(a.Right, b.Left)
	}
	return check(root, root)
}
```

## 相同树对比

```go
func isSameTree(p, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil || p.Val != q.Val {
		return false
	}
	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}
```

## 为什么这样写

对称不是左右子树相同。比如左子树的左孩子，应该和右子树的右孩子对称；左子树的右孩子，应该和右子树的左孩子对称。

递归函数一次拿两个节点，表示“这两个位置是否应该镜像相等”。空节点处理要先于取值比较。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：递归栈 $O(h)$。

## 易错点

- 把对称写成 `check(a.Left, b.Left)`。
- 一个空一个非空时没有立即失败。
- 只比较结构不比较值。
- `root == nil` 的情况没有考虑；上面写法会返回 true。

## 练习顺序

建议按这个顺序刷：#100, #101, #226。
