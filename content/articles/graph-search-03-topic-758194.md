---
title: 拓扑排序检测环：搜索与图论训练题解
category: 搜索与图论
summary: 有向依赖图能否完成，核心是有没有环；Kahn 算法不断删除入度为 0 的点，最后看是否删完。
problem_ids: [207, 210]
order: 103
---

# 拓扑排序检测环：搜索与图论训练题解

课程表这类题是有向图依赖问题。只要依赖中存在环，就不可能完成所有任务；没有环就能得到一个拓扑顺序。

一句话记法：**入度为 0 的点先做，做完它会降低后继入度。**

## 适用场景

- 课程表。
- 任务调度。
- 构建依赖。
- 有向图判环并输出顺序。

无向图判环不能直接用拓扑排序。

## Go 参考实现：课程表 II

```go
func findOrder(numCourses int, prerequisites [][]int) []int {
	graph := make([][]int, numCourses)
	indeg := make([]int, numCourses)
	for _, p := range prerequisites {
		a, b := p[0], p[1]
		graph[b] = append(graph[b], a)
		indeg[a]++
	}
	q := []int{}
	for i := 0; i < numCourses; i++ {
		if indeg[i] == 0 {
			q = append(q, i)
		}
	}
	order := []int{}
	for head := 0; head < len(q); head++ {
		u := q[head]
		order = append(order, u)
		for _, v := range graph[u] {
			indeg[v]--
			if indeg[v] == 0 {
				q = append(q, v)
			}
		}
	}
	if len(order) != numCourses {
		return nil
	}
	return order
}
```

## 为什么这样写

入度表示还有多少前置依赖没完成。入度为 0 的课可以立刻学习；学完后，它指向的后续课程少一个依赖。

如果最后还有节点没进入答案，说明这些节点互相依赖成环，没有任何一个能先开始。

## 复杂度

- 时间复杂度：$O(V+E)$。
- 空间复杂度：$O(V+E)$。

## 易错点

- 边方向写反。`[a,b]` 表示先 b 后 a，所以边是 `b -> a`。
- 只判断初始是否有入度为 0，不持续更新。
- 有环时仍返回部分顺序。
- 重复边可能导致入度重复，按题目约束决定是否去重。

## 练习顺序

建议按这个顺序刷：#207, #210。
