---
title: 分割回文串预处理判断：回溯训练题解
category: 回溯
summary: 分割回文串的搜索对象是切割点；先预处理任意区间是否回文，回溯时才能把重点放在“下一刀切在哪里”。
problem_ids: [131]
order: 111
---

# 分割回文串预处理判断：回溯训练题解

分割回文串不是枚举所有字符串组合，而是在原字符串上决定切割点。每次从 `start` 开始，枚举一个结束位置 `end`，如果 `s[start..end]` 是回文，就把这一段加入路径。

一句话记法：**每层切出一个回文段，下一层从切口后面继续。**

## 适用场景

适合这种写法的题：

- 要把一个字符串切成若干段。
- 每一段都要满足某个性质，比如回文、合法数字、字典词。
- 答案是所有可行分割方案。
- 区间合法性可以提前预处理，或快速判断。

如果只问最少切几刀，那就更偏动态规划；回溯适合输出所有方案。

## 图解思路

以 `s = "aab"` 为例：

```mermaid
flowchart TB
  A["start=0 path=[]"] --> B[切 a, start=1]
  A --> C[切 aa, start=2]
  B --> D[切 a, start=2]
  D --> E[切 b, 得到 a|a|b]
  C --> F[切 b, 得到 aa|b]
```

`"ab"` 不是回文，所以从 `start=1` 不会切出 `"ab"` 这条分支。

## 不变量

- `start` 表示下一段必须从 `s[start]` 开始。
- `path` 中的所有片段都已经确认是回文。
- `path` 拼起来正好等于 `s[0..start]`。
- 当 `start == s.len()` 时，整串已经被合法分割，可以收集答案。

## 手写步骤

1. 预处理 `pal[i][j]`，表示闭区间 `s[i..=j]` 是否回文。
2. 定义 `dfs(start)`。
3. 如果 `start == n`，复制 `path`。
4. 枚举 `end` 从 `start` 到 `n - 1`。
5. 如果 `pal[start][end]` 为假，跳过。
6. 选择 `s[start..=end]`，递归 `dfs(end + 1)`，然后撤销。

## Go 参考实现

```go
func partition(s string) [][]string {
	n := len(s)
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			pal[i][j] = s[i] == s[j] && (j-i < 2 || pal[i+1][j-1])
		}
	}

	ans := [][]string{}
	path := []string{}
	var dfs func(start int)
	dfs = func(start int) {
		if start == n {
			ans = append(ans, append([]string(nil), path...))
			return
		}

		for end := start; end < n; end++ {
			if !pal[start][end] {
				continue
			}
			path = append(path, s[start:end+1])
			dfs(end + 1)
			path = path[:len(path)-1]
		}
	}

	dfs(0)
	return ans
}
```

## Rust 参考实现

```rust
pub fn partition(s: String) -> Vec<Vec<String>> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut pal = vec![vec![false; n]; n];

    for i in (0..n).rev() {
        for j in i..n {
            pal[i][j] = bytes[i] == bytes[j] && (j - i < 2 || pal[i + 1][j - 1]);
        }
    }

    fn dfs(
        start: usize,
        s: &str,
        pal: &[Vec<bool>],
        path: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if start == s.len() {
            ans.push(path.clone());
            return;
        }

        for end in start..s.len() {
            if !pal[start][end] {
                continue;
            }
            path.push(s[start..=end].to_string());
            dfs(end + 1, s, pal, path, ans);
            path.pop();
        }
    }

    let mut path = Vec::new();
    let mut ans = Vec::new();
    dfs(0, &s, &pal, &mut path, &mut ans);
    ans
}
```

## 为什么这样写

如果每次切一段都临时双指针判断回文，代码也能过，但会把大量重复区间反复检查。`pal[i][j]` 的转移很简单：两端字符相同，并且中间也是回文。

回溯本身只负责枚举切割点。`start` 之前的内容已经被 `path` 完整覆盖，所以每层只需要决定下一段切到哪里。这个不变量能防止出现跳字符或重复字符的问题。

## 复杂度

- 预处理回文表是 $O(n^2)$ 时间和空间。
- 分割方案数量最坏是指数级，例如全是相同字符时分支很多。
- 不计输出，递归深度最多是 $O(n)$。

## 易错点

- `pal` 填表方向写错，导致 `pal[i+1][j-1]` 还没算出来。
- 终止条件写成 `start == n - 1`，漏掉最后一段。
- 字符串切片下标没有对齐字符边界；LeetCode 本题通常是小写英文，按字节安全。
- 收集答案时没有复制 `path`。

## 练习顺序

建议先刷 #131。

复盘时重点讲清楚两件事：`path` 覆盖的是 `s[0..start]`，以及 `pal` 表为什么要从后往前填。
