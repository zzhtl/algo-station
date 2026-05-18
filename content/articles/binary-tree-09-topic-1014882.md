---
title: 序列化保留空节点：二叉树训练题解
category: 二叉树
summary: 二叉树序列化必须保留空节点标记，否则不同结构可能得到同一串遍历结果，无法唯一反序列化。
problem_ids: [297, 449]
order: 109
---

# 序列化保留空节点：二叉树训练题解

只记录非空节点的前序或层序遍历，不能唯一还原一棵普通二叉树。必须把空孩子也记录下来，结构信息才完整。

一句话记法：**值记录节点，`#` 记录空指针。**

## 适用场景

- 序列化和反序列化二叉树。
- 判断两棵树结构是否相同。
- 需要把树结构转成字符串保存或传输。

BST 可以利用范围或有序性质压缩空节点，但普通二叉树不能省。

## 前序序列化

```go
func serialize(root *TreeNode) string {
	vals := []string{}
	var dfs func(*TreeNode)
	dfs = func(node *TreeNode) {
		if node == nil {
			vals = append(vals, "#")
			return
		}
		vals = append(vals, strconv.Itoa(node.Val))
		dfs(node.Left)
		dfs(node.Right)
	}
	dfs(root)
	return strings.Join(vals, ",")
}
```

## 前序反序列化

```go
func deserialize(data string) *TreeNode {
	vals := strings.Split(data, ",")
	idx := 0
	var build func() *TreeNode
	build = func() *TreeNode {
		if vals[idx] == "#" {
			idx++
			return nil
		}
		v, _ := strconv.Atoi(vals[idx])
		idx++
		node := &TreeNode{Val: v}
		node.Left = build()
		node.Right = build()
		return node
	}
	return build()
}
```

## 为什么这样写

前序遍历的顺序是“根、左、右”。反序列化时也按这个顺序消费 token：先建根，再建左子树，再建右子树。遇到 `#` 表示这棵子树为空，递归返回。

空节点标记是关键。例如只有左孩子和只有右孩子的树，如果不记录空节点，前序值序列可能一样，无法还原结构。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(n)$，包括输出字符串和递归栈。

## 易错点

- 不记录空节点，导致结构丢失。
- 反序列化时 `idx` 没有随消费 token 推进。
- 分隔符处理不一致，末尾多逗号导致空 token。
- Go 代码需要引入 `strings` 和 `strconv`。

## 练习顺序

建议按这个顺序刷：#297, #449。

先用普通二叉树练空节点标记，再看 BST 如何利用值域性质优化。
