---
title: 括号生成维护左右余额：回溯训练题解
category: 回溯
summary: 括号生成不是暴力拼字符串再校验，而是在生成过程中维护左右括号数量，让每一步都保持前缀合法。
problem_ids: [22]
order: 105
---

# 括号生成维护左右余额：回溯训练题解

括号生成的核心是前缀合法性。任意时刻，已经放入的右括号数量不能超过左括号数量；否则这个前缀无论后面怎么补，都不可能变成合法括号串。

一句话记法：**左括号没用完就可以放；右括号只有在不会超过左括号时才能放。**

## 适用场景

这类题通常满足：

- 答案是按位置逐步构造出来的字符串。
- 每一步有少量固定选择，比如放 `'('` 或 `')'`。
- 部分前缀一旦非法，后续不可能修复。
- 目标是输出所有合法构造。

这类题的重点不是 `used` 或 `start`，而是用状态变量维护“当前前缀是否仍可能合法”。

## 图解思路

以 `n = 3` 为例：

```mermaid
flowchart TB
  A["\"\" left=0 right=0"] --> B["( left=1 right=0"]
  B --> C["(( left=2 right=0"]
  B --> D["() left=1 right=1"]
  C --> E["((( left=3 right=0"]
  C --> F["(() left=2 right=1"]
  D --> G["()( left=2 right=1"]
  E --> H["((() left=3 right=1"]
  F --> I["(()) / (()("]
```

任何节点都不能让 `right > left`。比如空串后面不能先放 `')'`，因为这个前缀已经不可修复。

## 不变量

- `left` 表示已经放了多少个左括号。
- `right` 表示已经放了多少个右括号。
- 始终满足 `0 <= right <= left <= n`。
- 当字符串长度达到 `2*n` 时，必然有 `left == right == n`，可以收集答案。

也可以用剩余数量表示：`open` 表示还可以放多少左括号，`close` 表示还可以放多少右括号。无论哪种写法，都要表达同一个不变量：前缀必须合法。

## 手写步骤

1. 准备可变字符数组或字符串 `path`。
2. 如果长度是 `2*n`，复制为答案。
3. 如果 `left < n`，可以放 `'('`。
4. 如果 `right < left`，可以放 `')'`。
5. 每次递归返回后撤销刚才放入的字符。

## Go 参考实现

```go
func generateParenthesis(n int) []string {
	ans := []string{}
	path := []byte{}

	var dfs func(left, right int)
	dfs = func(left, right int) {
		if len(path) == 2*n {
			ans = append(ans, string(append([]byte(nil), path...)))
			return
		}

		if left < n {
			path = append(path, '(')
			dfs(left+1, right)
			path = path[:len(path)-1]
		}
		if right < left {
			path = append(path, ')')
			dfs(left, right+1)
			path = path[:len(path)-1]
		}
	}

	dfs(0, 0)
	return ans
}
```

## Rust 参考实现

```rust
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn dfs(n: usize, left: usize, right: usize, path: &mut String, ans: &mut Vec<String>) {
        if path.len() == 2 * n {
            ans.push(path.clone());
            return;
        }

        if left < n {
            path.push('(');
            dfs(n, left + 1, right, path, ans);
            path.pop();
        }
        if right < left {
            path.push(')');
            dfs(n, left, right + 1, path, ans);
            path.pop();
        }
    }

    let mut path = String::new();
    let mut ans = Vec::new();
    dfs(n as usize, 0, 0, &mut path, &mut ans);
    ans
}
```

## 为什么这样写

暴力做法会枚举长度为 `2n` 的所有括号串，再检查是否合法，候选数量是 `2^(2n)`。回溯剪枝的价值在于：一旦某个前缀右括号更多，这个前缀就永远不可能合法，可以马上停止。

`right < left` 是这道题最重要的条件。它不是为了让最终数量相等，而是为了保证每个前缀都合法。最终数量相等由 `left <= n` 和总长度 `2n` 自然保证。

## 复杂度

- 合法括号串数量是第 `n` 个卡特兰数，生成答案需要按输出规模计算。
- 每个答案长度为 `2n`，复制成本是 $O(n)$。
- 不计输出，递归深度和路径长度是 $O(n)$。

## 易错点

- 先生成所有字符串再判断，浪费大量非法分支。
- 写成 `right <= left` 后再放右括号，导致出现 `right == left` 时继续放 `')'` 的非法前缀。
- 只判断左右括号总数，没有维护前缀合法性。
- Rust 中频繁拼接新 `String`，比可变 `path` 回溯更重。

## 练习顺序

建议先刷 #22。

复盘时不要只背代码，重点说清楚两个剪枝条件：`left < n` 控制左括号总数，`right < left` 控制前缀合法性。
