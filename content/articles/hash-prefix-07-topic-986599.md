---
title: 滑窗哈希计数复用：哈希与前缀训练题解
category: 哈希与前缀
summary: 固定长度异位词窗口可以复用计数表；右端加入字符、左端移出字符，窗口签名实时更新。
problem_ids: [438, 567, 3]
order: 107
---

# 滑窗哈希计数复用：哈希与前缀训练题解

固定长度字符串匹配，不要每个窗口重新统计。滑动窗口每次只变动两个字符：右边新增，左边移出。

## Go 参考实现：找到所有异位词

```go
func findAnagrams(s string, p string) []int {
	if len(p) > len(s) {
		return nil
	}
	need, win := [26]int{}, [26]int{}
	for i := 0; i < len(p); i++ {
		need[p[i]-'a']++
		win[s[i]-'a']++
	}
	ans := []int{}
	if need == win {
		ans = append(ans, 0)
	}
	for r := len(p); r < len(s); r++ {
		win[s[r]-'a']++
		win[s[r-len(p)]-'a']--
		if need == win {
			ans = append(ans, r-len(p)+1)
		}
	}
	return ans
}
```

## 为什么这样写

异位词判断只关心字符计数。窗口长度固定为 `len(p)`，滑动一步时，旧窗口和新窗口只有两个位置不同，所以更新计数表是 $O(1)$。

#3 无重复字符的最长子串是可变窗口，哈希表存字符最近位置或窗口内计数。

## 易错点

- 每个窗口重新排序，复杂度高。
- 移出字符下标写错。
- 窗口长度还没达到就比较。
- 字符集不止小写字母时仍用 `[26]int`。

## 练习顺序

建议按这个顺序刷：#438, #567, #3。
