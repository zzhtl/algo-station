---
title: 迭代栈模拟中序：二叉树训练题解
category: 二叉树
summary: 中序迭代要先一路向左压栈，弹出时访问节点，再转向右子树；栈保存的是还没访问的祖先节点。
problem_ids: [94, 98, 230, 173]
order: 112
---

# 迭代栈模拟中序：二叉树训练题解

中序遍历顺序是左、根、右。迭代写法的关键是：先沿着左指针一路压栈，直到不能再走；弹出栈顶时才访问当前节点。

一句话记法：**一路向左压栈，弹出即访问，随后转向右子树。**

## 适用场景

- 非递归中序遍历。
- BST 第 k 小。
- BST 迭代器。
- 验证 BST 的迭代版本。

BST 相关题经常利用中序有序性。

## Go 参考实现：中序遍历

```go
func inorderTraversal(root *TreeNode) []int {
	ans := []int{}
	st := []*TreeNode{}
	cur := root
	for cur != nil || len(st) > 0 {
		for cur != nil {
			st = append(st, cur)
			cur = cur.Left
		}
		cur = st[len(st)-1]
		st = st[:len(st)-1]
		ans = append(ans, cur.Val)
		cur = cur.Right
	}
	return ans
}
```

## 第 k 小怎么改

每弹出一个节点，说明中序访问到一个值。计数到 `k` 时返回：

```go
func kthSmallest(root *TreeNode, k int) int {
	st := []*TreeNode{}
	cur := root
	for cur != nil || len(st) > 0 {
		for cur != nil {
			st = append(st, cur)
			cur = cur.Left
		}
		cur = st[len(st)-1]
		st = st[:len(st)-1]
		k--
		if k == 0 {
			return cur.Val
		}
		cur = cur.Right
	}
	return -1
}
```

## 为什么这样写

根节点要等左子树全部访问完后才能访问。栈里保存的是“左路上暂时不能访问的祖先”。当左边走到底，栈顶就是下一个应该访问的节点。

访问完节点后，再处理它的右子树；右子树也按照同样规则一路向左。

## 复杂度

- 时间复杂度：$O(n)$，第 k 小是 $O(h+k)$。
- 空间复杂度：$O(h)$。

## 易错点

- 根节点一开始就访问，写成前序。
- 弹出访问后忘记转向右子树。
- 循环条件只写 `cur != nil`，栈里剩余祖先丢失。
- BST 第 k 小把入栈次数当访问次数。

## 练习顺序

建议按这个顺序刷：#94, #230, #98, #173。
