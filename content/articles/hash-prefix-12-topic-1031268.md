---
title: 哈希缓存递归结果：哈希与前缀训练题解
category: 哈希与前缀
summary: 递归中重复状态可以用哈希表缓存；key 必须完整描述状态，value 存该状态的计算结果。
problem_ids: [139, 140, 494]
order: 112
---

# 哈希缓存递归结果：哈希与前缀训练题解

记忆化搜索本质是哈希缓存。递归函数的参数就是状态，返回值就是缓存值。

## Go 参考实现：单词拆分

```go
func wordBreak(s string, wordDict []string) bool {
	dict := map[string]bool{}
	for _, w := range wordDict {
		dict[w] = true
	}
	memo := map[int]bool{}
	seen := map[int]bool{}
	var dfs func(int) bool
	dfs = func(start int) bool {
		if start == len(s) {
			return true
		}
		if seen[start] {
			return memo[start]
		}
		seen[start] = true
		for end := start + 1; end <= len(s); end++ {
			if dict[s[start:end]] && dfs(end) {
				memo[start] = true
				return true
			}
		}
		memo[start] = false
		return false
	}
	return dfs(0)
}
```

## 为什么这样写

`dfs(start)` 表示 `s[start:]` 能否被拆分。不同拆分路径会反复问同一个 `start`，所以缓存它的结果能把指数级搜索降下来。

失败状态也要缓存，否则大量不可行后缀会被重复计算。

## 易错点

- key 没包含完整状态。
- 只缓存 true，不缓存 false。
- 用默认 false 无法区分“没算过”和“算过为 false”，需要 seen 或三态。
- 子串切片在某些语言里有额外拷贝成本。

## 练习顺序

建议按这个顺序刷：#139, #140, #494。
