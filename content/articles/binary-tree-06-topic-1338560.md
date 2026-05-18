---
title: 最近公共祖先向上汇报：二叉树训练题解
category: 二叉树
summary: LCA 递归的返回值表示当前子树是否找到了 p 或 q；左右都找到时，当前节点就是最近公共祖先。
problem_ids: [236, 235]
order: 106
---

# 最近公共祖先向上汇报：二叉树训练题解

最近公共祖先不是从根往下猜，而是让左右子树向上汇报有没有找到目标节点。当前节点根据左右汇报结果判断自己是不是答案。

一句话记法：**左边找到一个，右边找到一个，当前节点就是 LCA。**

## 适用场景

- 普通二叉树最近公共祖先。
- BST 最近公共祖先。
- 需要从子树向父节点汇报“是否找到目标”。

普通二叉树不能利用大小关系；BST 可以用值域方向优化。

## Go 参考实现：普通二叉树

```go
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil || root == p || root == q {
		return root
	}
	left := lowestCommonAncestor(root.Left, p, q)
	right := lowestCommonAncestor(root.Right, p, q)
	if left != nil && right != nil {
		return root
	}
	if left != nil {
		return left
	}
	return right
}
```

## BST 写法

```go
func lowestCommonAncestorBST(root, p, q *TreeNode) *TreeNode {
	for root != nil {
		if p.Val < root.Val && q.Val < root.Val {
			root = root.Left
		} else if p.Val > root.Val && q.Val > root.Val {
			root = root.Right
		} else {
			return root
		}
	}
	return nil
}
```

## 为什么这样写

普通二叉树的返回值有三种含义：

- `nil`：当前子树没有 p 或 q。
- `p/q`：当前子树找到了其中一个。
- 某个祖先节点：当前子树已经找到 LCA。

如果左右子树都非空，说明 p 和 q 分别在两边，当前节点就是最低的汇合点。如果只有一边非空，就把那边找到的结果继续向上交。

## 复杂度

- 普通二叉树时间复杂度 $O(n)$。
- BST 平均 $O(h)$。
- 空间复杂度为递归栈或迭代常数空间。

## 易错点

- 用节点值判断相等，题目通常要求节点引用。
- 找到 `p` 或 `q` 后还继续往下递归，逻辑复杂化。
- 普通二叉树误用 BST 大小关系。
- 不理解返回值含义，把“找到一个目标”和“找到 LCA”混在一起。

## 练习顺序

建议按这个顺序刷：#236, #235。
