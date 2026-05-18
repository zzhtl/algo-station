---
title: 栈模拟递归展开：栈与队列训练题解
category: 栈与队列
summary: 解码字符串这类嵌套结构，本质是遇到左括号保存外层状态，遇到右括号展开当前层并回到上一层。
problem_ids: [394]
order: 106
---

# 栈模拟递归展开：栈与队列训练题解

遇到嵌套结构时，栈可以模拟递归调用。每进入一层，就把外层状态压栈；当前层结束时，弹出外层状态并合并结果。

一句话记法：**左括号入栈保存现场，右括号出栈恢复现场。**

## 适用场景

- 解码字符串 `3[a2[c]]`。
- 嵌套括号结构。
- 需要把内层结果返回给外层。
- 递归写法能做，但迭代栈更直接。

## 图解思路

```mermaid
flowchart LR
  A[读到数字] --> B[累积 repeat]
  B --> C[读到 '[']
  C --> D[压入当前字符串和 repeat]
  D --> E[处理内层]
  E --> F[读到 ']']
  F --> G[弹出外层并重复拼接当前层]
```

栈中保存的是进入内层前的字符串和重复次数。

## 不变量

- `curr` 是当前层已经构造出的字符串。
- `num` 是当前刚读到的重复次数。
- 栈里保存外层的 `(prevString, repeat)`。
- 遇到 `]` 后，当前层变成外层的一部分。

## Go 参考实现

```go
func decodeString(s string) string {
	strStack := []string{}
	numStack := []int{}
	curr := ""
	num := 0

	for i := 0; i < len(s); i++ {
		ch := s[i]
		if ch >= '0' && ch <= '9' {
			num = num*10 + int(ch-'0')
		} else if ch == '[' {
			strStack = append(strStack, curr)
			numStack = append(numStack, num)
			curr = ""
			num = 0
		} else if ch == ']' {
			repeat := numStack[len(numStack)-1]
			numStack = numStack[:len(numStack)-1]
			prev := strStack[len(strStack)-1]
			strStack = strStack[:len(strStack)-1]
			part := ""
			for j := 0; j < repeat; j++ {
				part += curr
			}
			curr = prev + part
		} else {
			curr += string(ch)
		}
	}
	return curr
}
```

## 为什么这样写

`3[a2[c]]` 中，处理到第二个 `[` 时，外层已经有 `curr="a"`、`repeat=2`。内层算出 `"c"` 后，要先变成 `"cc"`，再拼回外层 `"a"`，得到 `"acc"`。最后再按外层的 `3` 重复。

栈保存现场，就是为了在内层结束后知道回到哪一层、重复几次。

## 复杂度

- 时间复杂度与最终输出长度相关。
- 空间复杂度：栈深度和输出字符串。

## 易错点

- 多位数字只读一位。
- 遇到 `[` 后没有清空当前层字符串。
- `]` 时重复次数和外层字符串弹出顺序弄错。
- 用字符串反复 `+=` 在某些语言中成本较高，工程里可用 builder 优化。

## 练习顺序

建议先刷 #394。

做完后可以用递归再写一版，对比“函数调用栈”和“手动栈”保存的状态完全一样。
