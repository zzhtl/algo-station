---
title: 字典序最小删除：贪心训练题解
category: 贪心
summary: 字典序贪心删除要在保证后面还能补齐的前提下，尽量弹出更大的栈顶，让更小字符靠前。
problem_ids: [316, 402, 1081]
order: 109
---

# 字典序最小删除：贪心训练题解

字典序最小类题，局部目标是让更小字符尽量靠前；但前提是弹掉的字符后面还能补回来。

一句话记法：**能补回来才敢弹，弹掉更大的栈顶。**

## Go 参考实现：去除重复字母

```go
func removeDuplicateLetters(s string) string {
	last := [26]int{}
	for i := range s {
		last[s[i]-'a'] = i
	}
	st := []byte{}
	seen := [26]bool{}
	for i := range s {
		c := s[i]
		idx := c - 'a'
		if seen[idx] {
			continue
		}
		for len(st) > 0 && c < st[len(st)-1] && last[st[len(st)-1]-'a'] > i {
			seen[st[len(st)-1]-'a'] = false
			st = st[:len(st)-1]
		}
		st = append(st, c)
		seen[idx] = true
	}
	return string(st)
}
```

## 为什么这样写

栈维护当前答案前缀。当前字符 `c` 比栈顶更小，若栈顶字符后面还会出现，那么把栈顶延后不会丢字符，还能让字典序变小，所以应该弹。

如果栈顶后面不再出现，就不能弹；否则最终答案会缺字符。

## 复杂度

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(字符集大小)$。

## 易错点

- 只要当前更小就弹，没有检查后面还能补回来。
- 没有 `seen`，同一字符重复入栈。
- #402 删除 K 位数字还要处理前导零和剩余删除次数。
- 相同字符弹不弹条件写错。

## 练习顺序

建议按这个顺序刷：#316, #1081, #402。
