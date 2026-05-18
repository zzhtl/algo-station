---
title: 计数表做多集合比较：哈希与前缀训练题解
category: 哈希与前缀
summary: 多集合比较不能只看元素是否存在，还要看每个元素出现次数；字符集小时优先用定长数组计数。
problem_ids: [242, 383, 350]
order: 114
---

# 计数表做多集合比较：哈希与前缀训练题解

多集合比较关心“有哪些元素”和“每个元素有几个”。异位词、赎金信、数组交集 II 都是这个模型。

## Go 参考实现：赎金信

```go
func canConstruct(ransomNote string, magazine string) bool {
	cnt := [26]int{}
	for i := 0; i < len(magazine); i++ {
		cnt[magazine[i]-'a']++
	}
	for i := 0; i < len(ransomNote); i++ {
		idx := ransomNote[i] - 'a'
		cnt[idx]--
		if cnt[idx] < 0 {
			return false
		}
	}
	return true
}
```

## 为什么这样写

如果字符集固定为小写字母，数组比哈希表更简单也更快。先统计供给方，再消耗需求方；一旦某个计数变负，说明供给不足。

数组交集 II 也是同理：对较短数组计数，扫描另一个数组时有库存才加入答案并减一。

## 易错点

- 用 set 判断，忽略重复次数。
- 字符集不固定却写死 `[26]int`。
- 消耗后没减计数，重复使用同一个字符。
- 两个数组交集 II 没考虑输出重复元素。

## 练习顺序

建议按这个顺序刷：#242, #383, #350。
