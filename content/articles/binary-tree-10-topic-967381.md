---
title: 从遍历序列重建树：二叉树训练题解
category: 二叉树
summary: 前序确定根，中序切分左右子树；用哈希表定位根在中序中的位置，就能递归构造整棵树。
problem_ids: [105, 106]
order: 110
---

# 从遍历序列重建树：二叉树训练题解

从遍历序列构造二叉树，关键是利用不同遍历的分工：前序或后序告诉你根是谁，中序告诉你左右子树的范围。

一句话记法：**前序第一个是根，中序根左边是左子树，右边是右子树。**

## 适用场景

- 前序 + 中序构造树。
- 中序 + 后序构造树。
- 节点值不重复。

如果没有中序，一般无法唯一确定普通二叉树结构。

## Go 参考实现：前序 + 中序

```go
func buildTree(preorder []int, inorder []int) *TreeNode {
	pos := map[int]int{}
	for i, v := range inorder {
		pos[v] = i
	}

	var build func(preL, preR, inL, inR int) *TreeNode
	build = func(preL, preR, inL, inR int) *TreeNode {
		if preL > preR {
			return nil
		}
		rootVal := preorder[preL]
		rootIdx := pos[rootVal]
		leftSize := rootIdx - inL
		root := &TreeNode{Val: rootVal}
		root.Left = build(preL+1, preL+leftSize, inL, rootIdx-1)
		root.Right = build(preL+leftSize+1, preR, rootIdx+1, inR)
		return root
	}
	return build(0, len(preorder)-1, 0, len(inorder)-1)
}
```

## 为什么这样写

前序区间 `[preL, preR]` 的第一个元素是当前子树根。找到它在中序里的位置 `rootIdx` 后：

- 中序 `[inL, rootIdx-1]` 是左子树。
- 中序 `[rootIdx+1, inR]` 是右子树。
- 左子树大小是 `rootIdx - inL`。

知道左子树大小，就能把前序区间也切成左右两段。

## 复杂度

- 时间复杂度：$O(n)$，哈希表让每个根定位 $O(1)$。
- 空间复杂度：$O(n)$。

## 易错点

- 每次在线性扫描中序找根，退化成 $O(n^2)$。
- 左子树大小算错，前序区间切歪。
- 区间开闭约定混乱。
- 节点值重复时哈希表定位不唯一。

## 练习顺序

建议按这个顺序刷：#105, #106。

先掌握前序 + 中序，再把“根在后序最后一个”迁移到中序 + 后序。
