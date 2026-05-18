---
title: 哈希表做双向映射：哈希与前缀训练题解
category: 哈希与前缀
summary: 同构字符串这类题要保证一一映射；只做单向 map 会漏掉两个字符映到同一个字符的冲突。
problem_ids: [205, 290]
order: 105
---

# 哈希表做双向映射：哈希与前缀训练题解

同构关系要求一一对应：`s` 中同一个字符必须映到同一个字符，同时不同字符不能映到同一个目标字符。

一句话记法：**正向要一致，反向也要唯一。**

## Go 参考实现

```go
func isIsomorphic(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	m1 := map[byte]byte{}
	m2 := map[byte]byte{}
	for i := 0; i < len(s); i++ {
		a, b := s[i], t[i]
		if v, ok := m1[a]; ok && v != b {
			return false
		}
		if v, ok := m2[b]; ok && v != a {
			return false
		}
		m1[a] = b
		m2[b] = a
	}
	return true
}
```

## 为什么这样写

只检查 `s -> t` 会让 `"ab"` 和 `"cc"` 误判为真，因为 `a->c`、`b->c` 在单向表里都不冲突。但反向 `c` 同时对应 `a` 和 `b`，违反一一映射。

也可以记录每个字符上次出现位置，两个字符串当前位置的上次出现位置必须相同。

## 易错点

- 只做单向映射。
- 映射已经存在时直接覆盖。
- 字符串长度不同没先判断。
- 单词模式 #290 中还要先 split 单词。

## 练习顺序

建议按这个顺序刷：#205, #290。
