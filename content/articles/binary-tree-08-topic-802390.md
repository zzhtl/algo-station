---
title: 路径和回溯撤销：二叉树训练题解
category: 二叉树
summary: 路径和需要维护从根到当前节点的状态；进入节点更新路径，离开节点撤销，避免污染兄弟分支。
problem_ids: [112, 113, 437]
order: 108
---

# 路径和回溯撤销：二叉树训练题解

二叉树路径题常同时有递归和回溯。只要你维护了可变路径或前缀计数，递归返回时就要撤销当前节点造成的影响。

一句话记法：**当前节点加进去，左右子树跑完，再把当前节点拿出来。**

## 适用场景

- 输出所有根到叶路径。
- 统计路径和。
- 使用前缀和统计任意向下路径。

只传不可变的剩余值时不需要撤销；共享可变状态时必须撤销。

## Go 参考实现：路径总和 II

```go
func pathSum(root *TreeNode, targetSum int) [][]int {
	ans := [][]int{}
	path := []int{}
	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, remain int) {
		if node == nil {
			return
		}
		path = append(path, node.Val)
		remain -= node.Val
		if node.Left == nil && node.Right == nil && remain == 0 {
			ans = append(ans, append([]int(nil), path...))
		}
		dfs(node.Left, remain)
		dfs(node.Right, remain)
		path = path[:len(path)-1]
	}
	dfs(root, targetSum)
	return ans
}
```

## 前缀和统计路径

#437 统计任意向下路径和时，前缀表也是共享状态：

```go
func pathSumCount(root *TreeNode, target int) int {
	cnt := map[int]int{0: 1}
	ans := 0
	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, sum int) {
		if node == nil {
			return
		}
		sum += node.Val
		ans += cnt[sum-target]
		cnt[sum]++
		dfs(node.Left, sum)
		dfs(node.Right, sum)
		cnt[sum]--
	}
	dfs(root, 0)
	return ans
}
```

## 为什么这样写

路径状态只对当前根到节点这一条路径有效。左子树访问结束后，如果不撤销，右子树会看到左子树节点，路径就不再真实。

前缀和表也是同理：它记录的是当前递归路径上的前缀和次数，不是全树全局次数。

## 复杂度

- #113 时间复杂度 $O(n \cdot h)$，因为复制路径有成本。
- #437 时间复杂度 $O(n)$。
- 空间复杂度：递归栈和路径/前缀表 $O(h)$。

## 易错点

- 保存答案时没有复制 path。
- 非叶子节点命中 target 就收集。
- 前缀和表递归退出时没有 `cnt[sum]--`。
- 把“从根开始路径”和“任意向下路径”混成一种写法。

## 练习顺序

建议按这个顺序刷：#112, #113, #437。
