---
title: 异位词签名分组：哈希与前缀训练题解
category: 哈希与前缀
summary: 分组题要先设计签名 key；异位词可以用排序字符串或 26 维计数向量作为等价类签名。
problem_ids: [49, 242, 438]
order: 102
---

# 异位词签名分组：哈希与前缀训练题解

异位词的本质是字符多集合相同。分组时只要给每个字符串算出同一个签名，就能用哈希表聚合。

一句话记法：**同类对象先变成同一个 key。**

## Go 参考实现：字母异位词分组

```go
func groupAnagrams(strs []string) [][]string {
	groups := map[[26]int][]string{}
	for _, s := range strs {
		key := [26]int{}
		for i := 0; i < len(s); i++ {
			key[s[i]-'a']++
		}
		groups[key] = append(groups[key], s)
	}
	ans := [][]string{}
	for _, g := range groups {
		ans = append(ans, g)
	}
	return ans
}
```

## 为什么这样写

排序字符串如 `"eat" -> "aet"` 也能做 key，复杂度是每个字符串 $O(L \log L)$。计数向量在字符集固定时是 $O(L)$，更直接表达“每个字符出现几次”。

找所有异位词 #438 则把这个计数签名放进滑动窗口里维护。

## 易错点

- 用字符集合而不是计数，无法区分 `"ab"` 和 `"aab"`。
- key 使用切片，Go 中切片不能直接作为 map key。
- 字符集不是小写字母时仍然用 `[26]int`。
- 分组结果顺序通常无要求，不要过度处理。

## 练习顺序

建议按这个顺序刷：#242, #49, #438。
