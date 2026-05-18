---
title: 并查集维护连通块：搜索与图论训练题解
category: 搜索与图论
summary: 并查集适合动态合并和查询连通性；每次 union 成功，连通块数量减少一。
problem_ids: [547, 684, 200, 990, 1971]
order: 112
---

# 并查集维护连通块：搜索与图论训练题解

并查集只解决一类问题：合并集合、查询两个点是否属于同一集合。它不关心路径具体怎么走，只关心连通性。

一句话记法：**find 找代表元，union 合并集合。**

## Go 参考实现

```go
type DSU struct {
	parent []int
	size   []int
	count  int
}

func NewDSU(n int) *DSU {
	p, s := make([]int, n), make([]int, n)
	for i := 0; i < n; i++ {
		p[i], s[i] = i, 1
	}
	return &DSU{parent: p, size: s, count: n}
}

func (d *DSU) Find(x int) int {
	if d.parent[x] != x {
		d.parent[x] = d.Find(d.parent[x])
	}
	return d.parent[x]
}

func (d *DSU) Union(a, b int) bool {
	ra, rb := d.Find(a), d.Find(b)
	if ra == rb {
		return false
	}
	if d.size[ra] < d.size[rb] {
		ra, rb = rb, ra
	}
	d.parent[rb] = ra
	d.size[ra] += d.size[rb]
	d.count--
	return true
}
```

## 为什么这样写

`Find` 返回一个集合的代表元。两个点连通，当且仅当代表元相同。路径压缩让以后查找更快；按大小合并避免树太深。

`Union` 返回 false 表示两个点本来就在同一集合，这在冗余连接、成环检测里很有用。

## 复杂度

- 路径压缩 + 按大小合并后，摊还近似 $O(1)$。
- 空间复杂度：$O(n)$。

## 易错点

- 合并时直接 `parent[a] = b`，没有先找根。
- 连通块数量在重复 union 时也减少。
- 二维网格转编号时公式写错：`id = r*n + c`。
- 并查集不能回答最短路径长度。

## 练习顺序

建议按这个顺序刷：#547, #1971, #684, #990, #200。
