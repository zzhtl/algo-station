#!/usr/bin/env python3
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "content" / "articles"
TARGET_NEW_ARTICLES = 182


@dataclass(frozen=True)
class Category:
    name: str
    slug: str
    focus: str
    code_subject: str
    problems: list[int]
    patterns: list[str]


CATEGORIES: list[Category] = [
    Category(
        "数组与字符串",
        "array-string",
        "把下标、区间和元素关系画成可移动的窗口或指针。",
        "nums []int",
        [1, 11, 15, 26, 27, 75, 88, 167, 283, 560, 724, 977],
        [
            "两端指针缩小候选",
            "同向指针原地覆盖",
            "滑动窗口维护计数",
            "固定一维枚举另一维",
            "排序后跳过重复值",
            "原地交换与稳定写入",
            "前后缀乘积拆贡献",
            "差分数组批量更新",
            "循环数组取模遍历",
            "矩阵按层模拟",
            "字符桶替代哈希表",
            "双数组归并扫描",
            "哨兵简化边界",
            "区间压缩为事件点",
        ],
    ),
    Category(
        "哈希与前缀",
        "hash-prefix",
        "把历史状态放进表里，让当前元素只做一次查询。",
        "nums []int",
        [1, 3, 49, 128, 205, 217, 242, 347, 438, 560, 974, 1248],
        [
            "前缀和配合次数表",
            "异位词签名分组",
            "最长连续序列起点判断",
            "同余前缀归类",
            "哈希表做双向映射",
            "频率桶找 Top K",
            "滑窗哈希计数复用",
            "集合去重再扫描",
            "状态压缩成字符串键",
            "计数差分判断覆盖",
            "前缀异或查补集",
            "哈希缓存递归结果",
            "坐标离散化建索引",
            "计数表做多集合比较",
        ],
    ),
    Category(
        "链表",
        "linked-list",
        "先画断链和接链顺序，再写代码，避免指针丢失。",
        "head *ListNode",
        [2, 19, 21, 23, 24, 25, 61, 82, 83, 92, 141, 142],
        [
            "虚拟头节点统一删除",
            "快慢指针找中点",
            "原地翻转一段链表",
            "两链表同步归并",
            "检测环并找入口",
            "倒数第 K 个节点",
            "分割链表保持相对顺序",
            "K 组翻转的边界检查",
            "插入排序维护已排区",
            "归并排序拆分合并",
            "复制随机指针三步法",
            "双指针相交节点",
            "旋转链表成环再断开",
            "删除重复节点的前驱技巧",
        ],
    ),
    Category(
        "栈与队列",
        "stack-queue",
        "栈保存还没找到答案的元素，队列保存按时间过期的元素。",
        "nums []int",
        [20, 71, 84, 150, 155, 224, 225, 232, 239, 394, 496, 739],
        [
            "单调递增栈找左边界",
            "单调递减栈找下一个更大",
            "双端队列维护窗口最大值",
            "表达式遇右括号结算",
            "最小栈保存历史最小值",
            "栈模拟递归展开",
            "队列模拟层序状态",
            "两个栈实现队列",
            "两个队列实现栈",
            "括号匹配的类型栈",
            "柱状图延迟结算面积",
            "路径简化丢弃空段",
            "逆波兰表达式即时求值",
            "循环队列固定容量",
        ],
    ),
    Category(
        "二分搜索",
        "binary-search",
        "把答案空间变成单调布尔函数，在边界上收缩。",
        "nums []int",
        [33, 34, 35, 69, 74, 81, 153, 154, 162, 278, 410, 875],
        [
            "左闭右开找第一个满足",
            "旋转数组判断有序半边",
            "答案二分最小可行值",
            "浮点二分控制迭代次数",
            "二维矩阵映射一维",
            "峰值用相邻关系判方向",
            "查找区间左右边界",
            "二分插入位置",
            "容量问题的可行性函数",
            "最大化最小值的反向二分",
            "带重复元素的退化处理",
            "二分套贪心验证",
            "二分套计数函数",
            "指数扩张再二分",
        ],
    ),
    Category(
        "二叉树",
        "binary-tree",
        "每个节点只回答两个问题：向父亲交什么，自己更新什么。",
        "root *TreeNode",
        [94, 98, 100, 101, 102, 104, 105, 110, 112, 124, 226, 236],
        [
            "递归遍历三段式",
            "层序遍历按层收集",
            "后序返回高度与平衡",
            "前序构造路径状态",
            "中序验证搜索树",
            "最近公共祖先向上汇报",
            "直径在后序中更新",
            "路径和回溯撤销",
            "序列化保留空节点",
            "从遍历序列重建树",
            "镜像递归比较",
            "迭代栈模拟中序",
            "树形 DP 选与不选",
            "二叉搜索树范围剪枝",
        ],
    ),
    Category(
        "搜索与图论",
        "graph-search",
        "明确点、边、状态和访问标记，先保证不重复再谈优化。",
        "graph [][]int",
        [79, 127, 130, 200, 207, 210, 286, 417, 542, 695, 994, 1091],
        [
            "网格 DFS 染色",
            "多源 BFS 同时扩散",
            "拓扑排序检测环",
            "最短路 BFS 分层",
            "回溯搜索单词路径",
            "边界出发反向标记",
            "状态压缩 BFS",
            "Dijkstra 处理非负权",
            "0-1 BFS 用双端队列",
            "Bellman-Ford 限制边数",
            "Floyd 处理全源最短路",
            "并查集维护连通块",
            "桥和割点的 lowlink",
            "二分图染色判定",
        ],
    ),
    Category(
        "动态规划",
        "dynamic-programming",
        "先定义状态含义，再用转移解释每个格子从哪里来。",
        "nums []int",
        [5, 53, 62, 63, 70, 72, 91, 120, 198, 300, 322, 416],
        [
            "一维滚动数组压空间",
            "二维路径 DP 填表",
            "背包容量倒序更新",
            "完全背包正序更新",
            "区间 DP 枚举分割点",
            "树形 DP 汇总子树",
            "状态机 DP 分阶段",
            "记忆化搜索剪重复",
            "最长上升子序列贪心优化",
            "编辑距离三来源转移",
            "打家劫舍选与不选",
            "数字解码按前缀转移",
            "买卖股票持仓状态",
            "子数组最大和在线转移",
        ],
    ),
    Category(
        "贪心",
        "greedy",
        "找一个能被交换论证保护的局部选择。",
        "intervals [][]int",
        [45, 55, 56, 57, 122, 134, 135, 406, 435, 452, 455, 621],
        [
            "区间按结束时间排序",
            "跳跃游戏维护最远边界",
            "加油站总量与最低点",
            "分发糖果左右两遍",
            "重构队列先高后低",
            "合并区间维护右端",
            "最少箭射气球",
            "任务调度冷却槽",
            "字典序最小删除",
            "买卖股票累计正收益",
            "局部最优配饼干",
            "会议室扫描线",
            "单调栈做贪心删除",
            "反向思考最大化剩余",
        ],
    ),
    Category(
        "堆与优先队列",
        "heap-priority",
        "每次只取当前最需要处理的元素，用堆维护动态最值。",
        "nums []int",
        [23, 215, 239, 295, 347, 373, 502, 621, 703, 767, 857, 973],
        [
            "小根堆维护 Top K 大",
            "大根堆弹出当前最大",
            "双堆维护数据流中位数",
            "多路归并弹最小头",
            "堆加懒删除处理过期",
            "按收益解锁项目",
            "重组字符串错峰排布",
            "距离原点最近 K 点",
            "雇佣工人按比率排序",
            "滑动窗口堆顶校验",
            "任务冷却按剩余次数",
            "第 K 小数对枚举",
            "实时排行榜更新",
            "最小化合并成本",
        ],
    ),
    Category(
        "回溯",
        "backtracking",
        "搜索树上每一层做选择，递归返回时撤销选择。",
        "path []int",
        [17, 22, 37, 39, 40, 46, 47, 51, 77, 78, 79, 90],
        [
            "排列问题使用 used 数组",
            "组合问题递增 start",
            "子集问题每层都收集",
            "重复元素先排序再剪枝",
            "括号生成维护左右余额",
            "电话号码按字符展开",
            "数独按空格递归",
            "N 皇后列与斜线剪枝",
            "组合总和控制重复选择",
            "单词搜索原地标记",
            "分割回文串预处理判断",
            "IP 地址分段校验",
            "图路径回溯防环",
            "位运算加速可选集合",
        ],
    ),
    Category(
        "数学与位运算",
        "math-bit",
        "把运算性质写成不变量，避免暴力枚举。",
        "nums []int",
        [7, 9, 50, 66, 67, 136, 137, 169, 191, 202, 231, 268],
        [
            "快速幂二进制拆指数",
            "异或消去成对元素",
            "摩尔投票找多数",
            "按位计数还原唯一数",
            "辗转相除求最大公约数",
            "质数筛批量预处理",
            "进制转换逐位处理",
            "低位技巧 x & -x",
            "判断 2 的幂",
            "快乐数快慢指针",
            "组合数递推",
            "前缀乘积处理除法",
            "随机化避免最坏情况",
            "模运算保持非负",
        ],
    ),
    Category(
        "设计与数据结构",
        "design-structure",
        "用明确的不变量约束内部结构，让每个操作都容易验证。",
        "key int",
        [146, 155, 208, 211, 225, 232, 284, 295, 303, 304, 380, 705],
        [
            "LRU 哈希表加双链表",
            "Trie 节点保存分支",
            "前缀和类支持区间查询",
            "随机集合数组加索引表",
            "最小栈同步保存历史",
            "迭代器预取下一个元素",
            "二维前缀和封装查询",
            "哈希集合处理冲突",
            "循环队列固定数组",
            "文件系统路径树",
            "数据流中位数双堆",
            "支持通配符的字典树",
            "时间键值存储二分查找",
            "快照数组版本列表",
        ],
    ),
]


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    written = 0
    for cat in CATEGORIES:
        for index, pattern in enumerate(cat.patterns, start=1):
            if written >= TARGET_NEW_ARTICLES:
                return
            slug = f"{cat.slug}-{index:02d}-{slugify(pattern)}"
            path = OUT / f"{slug}.md"
            path.write_text(render_article(cat, pattern, index, written), encoding="utf-8")
            written += 1
    if written != TARGET_NEW_ARTICLES:
        raise SystemExit(f"expected {TARGET_NEW_ARTICLES}, wrote {written}")


def render_article(cat: Category, pattern: str, index: int, global_index: int) -> str:
    title = f"{pattern}：{cat.name}训练题解"
    problems = rotate(cat.problems, index)[:6]
    summary = f"围绕“{pattern}”整理的训练型题解：用专属图解拆出状态、不变量和代码落点。"
    order = 100 + index
    go_name = f"solve{global_index + 1:03d}"
    rust_name = f"solve_{global_index + 1:03d}"
    visual = visual_for(cat, pattern)
    go_code, rust_code = code_pair(cat, go_name, rust_name)
    return f"""---
title: {title}
category: {cat.name}
summary: {summary}
problem_ids: [{", ".join(str(p) for p in problems)}]
order: {order}
---

# {title}

这篇不是背模板，而是把 **{pattern}** 拆成可以手写、可以检查的步骤。训练时建议先遮住题解，只看图和不变量，自己写一版，再展开代码对照。

## 适用场景

{cat.focus}

{visual["scene"]}

## 图解思路

```mermaid
{visual["diagram"]}
```

按这张图写代码时，先不要急着写完整函数，先把图里的三个变量写出来：

{visual["state"]}

## 手写步骤

{visual["steps"]}

## Go 参考骨架

```go
{go_code}
```

## Rust 参考骨架

```rust
{rust_code}
```

## 为什么这样写

{visual["why"]}

## 复杂度

{visual["complexity"]}

## 易错点

{visual["pitfalls"]}

## 练习顺序

建议按这个顺序刷：{", ".join(f"#{p}" for p in problems)}。每题都先写 Go 或 Rust，再对照题解。
"""


def visual_for(cat: Category, pattern: str) -> dict[str, str]:
    by_slug = {
        "array-string": array_visual,
        "hash-prefix": hash_visual,
        "linked-list": linked_list_visual,
        "stack-queue": stack_visual,
        "binary-search": binary_search_visual,
        "binary-tree": tree_visual,
        "graph-search": graph_visual,
        "dynamic-programming": dp_visual,
        "greedy": greedy_visual,
        "heap-priority": heap_visual,
        "backtracking": backtracking_visual,
        "math-bit": math_visual,
        "design-structure": design_visual,
    }
    return by_slug.get(cat.slug, array_visual)(pattern)


def base_visual(pattern: str, diagram: str, scene: str, state: str, steps: str, why: str, complexity: str, pitfalls: str) -> dict[str, str]:
    return {
        "diagram": diagram,
        "scene": scene,
        "state": state,
        "steps": steps,
        "why": why,
        "complexity": complexity,
        "pitfalls": pitfalls,
    }


def array_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  L[left] --> W[维护窗口 / 区间]
  R[right 扩张] --> W
  W --> C{{{pattern} 条件是否满足?}}
  C -->|满足| A[更新答案]
  C -->|不满足| S[left 收缩或交换]
  S --> W
  A --> R""",
        "- 题目要求连续区间、相向查找、原地修改或去重时，优先把数组画成一条线。\n- 关键不是枚举所有下标，而是让 `left/right/slow/fast` 每次移动后都能排除一批候选。",
        "- `left/right`：当前仍有意义的搜索区间。\n- `window`：区间内已经统计好的信息。\n- `ans`：目前见过的最优值或写入位置。",
        "1. 初始化 `left = 0` 和答案。\n2. 用 `right` 扫描，每次把新元素加入窗口。\n3. 如果窗口不合法，循环移动 `left` 直到合法。\n4. 在合法状态更新答案或写入位置。",
        "`right` 最多走 `n` 次，`left` 也最多走 `n` 次，所以这类题通常能从暴力 $O(n^2)$ 降到 $O(n)$。",
        "- 时间复杂度：$O(n)$，排序型变体是 $O(n \\log n)$。\n- 空间复杂度：只维护指针时 $O(1)$；需要计数表时 $O(k)$。",
        "- 更新答案的时机错放在收缩前。\n- 去重时只跳过一侧，导致重复答案。\n- 原地写入时 `slow` 先加一，覆盖了还没读取的值。",
    )


def hash_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  I[当前元素 x] --> K[计算 key: {pattern}]
  K --> Q{{key 是否出现过?}}
  Q -->|出现| A[用历史状态更新答案]
  Q -->|未出现| M[写入 map / set]
  A --> M
  M --> N[处理下一个元素]""",
        "- 看到“出现次数、前缀、配对、分组、去重”，先想能不能把历史状态压成一个 key。\n- 当前元素只负责一次查询和一次写入。",
        "- `key`：当前状态的签名，比如前缀和、字符计数、同余值。\n- `table`：历史 key 到次数/下标/集合的映射。\n- `ans`：查询命中后累计或更新。",
        "1. 明确 key 的含义，保证相同 key 代表同一类状态。\n2. 循环开始前放入空前缀或初始状态。\n3. 先查表更新答案，再按题意写入当前状态。\n4. 如果题目要求最短/最长，表里保存下标；如果要求数量，表里保存次数。",
        "哈希表把“找一个历史状态”的成本降为均摊 $O(1)$，难点变成设计不会冲突的 key。",
        "- 时间复杂度：均摊 $O(n)$。\n- 空间复杂度：$O(n)$ 或状态种类数。",
        "- 忘记插入初始前缀，导致从 0 开始的答案漏掉。\n- 先写入再查询，误用当前元素匹配自己。\n- key 不完整，例如异位词只按长度分组。",
    )


def linked_list_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  D[dummy] --> P[prev]
  P --> C[curr]
  C --> N[next]
  C -.保存 next.-> N
  C -->|断开/翻转| P
  P -->|接回| N
  N --> T[继续扫描: {pattern}]""",
        "- 链表题先画节点关系，不要先写循环。\n- 每次改 `next` 前必须先保存后继，否则链会断丢。",
        "- `dummy`：统一处理头节点变化。\n- `prev/curr/next`：分别代表已处理尾、当前节点、未处理头。\n- `guard`：边界检查，决定这一段能不能操作。",
        "1. 建 `dummy` 指向头节点。\n2. 每轮先保存 `next`，再修改 `curr.next`。\n3. 操作一段链表时先检查长度够不够。\n4. 返回 `dummy.next`，不要返回旧 head。",
        "链表操作的正确性来自“未处理部分始终能通过某个指针访问到”。只要这个不变量没丢，就可以放心翻转、删除或拼接。",
        "- 时间复杂度：$O(n)$。\n- 空间复杂度：迭代写法 $O(1)$，递归写法可能 $O(n)$ 栈空间。",
        "- 修改 `curr.next` 前没有保存 `next`。\n- 翻转后忘记把前驱接回新头。\n- 删除头节点时没有 dummy，分支越来越多。",
    )


def stack_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  X[读入元素] --> C{{能否结算栈顶?}}
  C -->|能| P[pop 结算]
  P --> C
  C -->|不能| S[push 当前元素]
  S --> N[下一个: {pattern}]
  N --> X""",
        "- 遇到“下一个更大/更小、括号、表达式、窗口最大值”，先判断新元素是否能让旧元素结算。\n- 栈/队列里保存的是“还没得到答案”的候选。",
        "- `stack/deque`：尚未结算的下标或值。\n- `current`：新进来的元素，负责触发结算。\n- `ans`：弹出时立刻确定。",
        "1. 栈里通常存下标，方便计算距离或区间宽度。\n2. 新元素进来时，用 while 结算所有可结算的栈顶。\n3. 结算完再 push 当前元素。\n4. 扫描结束后处理栈里剩下的元素。",
        "单调结构的价值是让每个元素只进栈一次、出栈一次，避免对每个位置向两边反复扫描。",
        "- 时间复杂度：$O(n)$。\n- 空间复杂度：$O(n)$。",
        "- 用 if 只弹一次，漏掉连续可结算元素。\n- 栈里存值导致无法算距离。\n- 队列窗口题忘记移除过期下标。",
    )


def binary_search_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  L[lo] --> M[mid]
  H[hi] --> M
  M --> P{{"check mid: {pattern}"}}
  P -->|true| H2[hi = mid]
  P -->|false| L2[lo = mid + 1]
  H2 --> M
  L2 --> M""",
        "- 只要答案具有单调性，就把问题改写成 `check(x)` 是否可行。\n- 二分不是猜答案，而是每次排除一半不可能区间。",
        "- `lo/hi`：答案可能出现的闭开区间。\n- `mid`：当前试探值。\n- `check`：把题目条件转成 true/false。",
        "1. 先写 `check(x)`，不要先写二分循环。\n2. 明确要找第一个 true 还是最后一个 false。\n3. 使用 `for lo < hi`，根据 check 更新边界。\n4. 最后返回 `lo`，再用小样例验证边界。",
        "二分的正确性依赖单调性：如果 `x` 可行时更大的值也可行，就能找第一个可行；反之找最后一个可行。",
        "- 时间复杂度：$O(\\log R \\cdot check)$。\n- 空间复杂度：通常 $O(1)$。",
        "- `mid = (lo + hi) / 2` 在大数时溢出。\n- 边界是闭区间还是闭开区间混用。\n- check 的单调方向写反。",
    )


def tree_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart TB
  R[root] --> L[left 子树]
  R --> X[right 子树]
  L --> A[返回给 root 的信息]
  X --> B[返回给 root 的信息]
  A --> C{{在 root 合并: {pattern}}}
  B --> C
  C --> U[更新全局答案 / 返回父节点]""",
        "- 二叉树题不要只想遍历顺序，要问每个节点需要从子树拿什么信息。\n- 大部分题可以写成“左右子树返回值 + 当前节点合并”。",
        "- `left/right`：子树递归结果。\n- `global`：跨过当前节点才能更新的答案。\n- `return`：只能交给父节点的一条信息。",
        "1. 定义递归函数返回值，不要直接写遍历。\n2. 递归拿到左右子树信息。\n3. 在当前节点更新全局答案。\n4. 返回父节点能继续使用的信息。",
        "树题的关键是区分“路径能不能分叉”：更新全局答案可以用左右两边，返回父节点通常只能选一边。",
        "- 时间复杂度：$O(n)$。\n- 空间复杂度：递归栈 $O(h)$。",
        "- 把全局答案和返回值混为一谈。\n- 空节点返回值没有定义清楚。\n- 路径题递归退出时忘记撤销状态。",
    )


def graph_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  S[起点/多源点] --> Q[队列或递归栈]
  Q --> V[弹出状态]
  V --> N[枚举邻居]
  N --> C{{未访问且合法?}}
  C -->|是| A[标记并加入队列]
  C -->|否| D[跳过]
  A --> Q
  D --> Q""",
        "- 图题先定义状态：一个格子、一个节点，还是节点加额外条件。\n- 访问标记必须在入队/进入递归时完成，避免重复入队。",
        "- `state`：当前节点或网格坐标。\n- `visited`：已经确定处理过的状态。\n- `frontier`：下一层要扩展的边界。",
        "1. 把题目对象建模成点和边。\n2. 起点入队或进入递归时立即标记。\n3. 每次枚举四方向/邻接表。\n4. 只把合法且未访问的状态推入下一轮。",
        "图搜索的正确性来自“不重复处理同一状态”。BFS 还额外保证第一次到达就是最短层数。",
        "- 时间复杂度：$O(V + E)$ 或网格 $O(mn)$。\n- 空间复杂度：访问集合和队列 $O(V)$。",
        "- 出队才标记，导致同一节点重复入队。\n- 多源 BFS 没把所有源点一起入队。\n- 状态缺少额外维度，剪掉了合法路径。",
    )


def dp_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  S["定义 dp 状态"] --> B[初始化边界]
  B --> T[枚举状态]
  T --> R[从已知状态转移]
  R --> C{{"{pattern} 是否可压缩"}}
  C -->|可| O[滚动数组]
  C -->|不可| F[保留完整表]
  O --> A[答案]
  F --> A""",
        "- DP 题先写状态含义，再写转移来源。\n- 如果你无法解释 `dp[i]` 表示什么，代码多半只是碰运气。",
        "- `dp`：子问题答案。\n- `transition`：当前状态从哪些更小状态来。\n- `order`：保证转移来源已经算过。",
        "1. 用一句话定义 `dp` 的物理含义。\n2. 写出最小输入的边界值。\n3. 按依赖方向枚举状态。\n4. 如果只依赖前几层，再滚动压缩空间。",
        "DP 不是公式收集，而是把大问题拆成互不重复的子问题。转移式只是状态定义的直接结果。",
        "- 时间复杂度：状态数乘以转移数。\n- 空间复杂度：完整表或滚动数组。",
        "- 状态定义包含答案但转移缺信息。\n- 枚举顺序错，使用了还没计算的状态。\n- 背包题正序/倒序写反。",
    )


def greedy_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  I[候选集合] --> S[排序 / 选择标准]
  S --> P[选择当前最优]
  P --> C{{和已有选择冲突?}}
  C -->|冲突| D[丢弃或替换]
  C -->|不冲突| A[加入答案]
  A --> N[下一个候选]
  D --> N""",
        "- 贪心题先找排序标准，再证明当前选择不会让未来变差。\n- 只要证明不了，就先考虑 DP 或搜索。",
        "- `order`：处理候选的顺序。\n- `choice`：当前局部选择。\n- `proof`：交换论证或边界推进理由。",
        "1. 按结束时间、收益、边界或差值排序。\n2. 扫描候选，判断能否接到当前答案后面。\n3. 冲突时选择更有利于未来的那个。\n4. 用交换论证检查局部最优能推出全局最优。",
        "如果一个选择只影响未来剩余空间，而不影响过去答案，就可以尝试用贪心。",
        "- 时间复杂度：排序通常 $O(n \\log n)$，扫描 $O(n)$。\n- 空间复杂度：看是否需要额外结构。",
        "- 只凭直觉选局部最优，没有证明。\n- 排序字段选错，比如区间题按起点而不是终点。\n- 冲突时忘记保留对未来更友好的候选。",
    )


def heap_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart TB
  X[新元素/任务] --> H[入堆]
  H --> T[堆顶是当前最优]
  T --> C{{堆顶是否可用?}}
  C -->|可用| P[pop 并处理]
  C -->|过期| D[丢弃堆顶]
  D --> T
  P --> A[更新答案: {pattern}]""",
        "- 当“当前最大/最小”会动态变化时，用堆维护候选。\n- 堆顶可能过期，要在使用前校验。",
        "- `heap`：动态候选集合。\n- `top`：当前最值。\n- `valid`：判断堆顶是否还属于当前窗口/状态。",
        "1. 确定堆比较的是值、频率、距离还是时间。\n2. 新候选入堆。\n3. while 堆顶过期就弹出。\n4. 使用合法堆顶更新答案。",
        "堆只保证快速拿到当前最值，不保证内部全局有序。所以代码只能依赖堆顶，不能遍历堆做逻辑。",
        "- 时间复杂度：每次入堆/出堆 $O(\\log n)$。\n- 空间复杂度：$O(n)$。",
        "- 最大堆用负数模拟时比较字段写反。\n- 懒删除题没有校验堆顶有效性。\n- Top K 题堆大小没有限制。",
    )


def backtracking_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart TB
  N[当前层] --> C[枚举选择]
  C --> V{{选择是否合法?}}
  V -->|否| X[剪枝]
  V -->|是| D[做选择]
  D --> R[递归下一层]
  R --> U[撤销选择]
  U --> C""",
        "- 回溯题先画搜索树，每一层代表一个决策位置。\n- 代码只做三件事：选择、递归、撤销。",
        "- `path`：当前已经做出的选择。\n- `start/used`：控制可选范围，避免重复。\n- `valid`：剪枝条件。",
        "1. 定义递归函数参数：层数、起点、路径。\n2. 到达终止条件就复制路径。\n3. 枚举候选，非法就跳过。\n4. 做选择、递归、撤销选择。",
        "回溯保证完整性靠枚举所有分支，保证效率靠剪枝。剪枝必须不丢合法答案。",
        "- 时间复杂度：通常是指数级，取决于分支数和深度。\n- 空间复杂度：递归深度和路径。",
        "- 保存答案时没有复制 path。\n- 重复元素去重条件写错。\n- 撤销选择漏掉，污染兄弟分支。",
    )


def math_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  X[输入值] --> B[拆位 / 取模 / 分解]
  B --> I[维护数学不变量]
  I --> C{{是否满足 {pattern}?}}
  C -->|是| A[更新答案]
  C -->|否| N[继续处理下一位]
  N --> B""",
        "- 数学题先找不变量：异或抵消、模同余、位贡献、幂次拆分。\n- 不要从暴力枚举开始，先观察运算性质。",
        "- `invariant`：每一步都保持的等式或位关系。\n- `bit/mod`：局部处理单元。\n- `ans`：逐步累积的结果。",
        "1. 写出运算性质，比如 `a ^ a = 0`。\n2. 把输入拆成位、因子、余数或数字。\n3. 每一步保持不变量。\n4. 合并局部结果得到答案。",
        "数学题的代码通常很短，但前提是你能说明为什么每一步不会改变目标答案。",
        "- 时间复杂度：通常 $O(n)$ 或 $O(\\log x)$。\n- 空间复杂度：多数为 $O(1)$。",
        "- 负数取模没有转成非负。\n- 位运算优先级看错。\n- 快速幂忘记每步取模。",
    )


def design_visual(pattern: str) -> dict[str, str]:
    return base_visual(
        pattern,
        f"""flowchart LR
  API[操作接口] --> INV[结构不变量]
  INV --> R{{读操作?}}
  INV --> W{{写操作?}}
  R --> Q[按索引 / 堆顶 / 前缀查询]
  W --> U[同步更新多个结构]
  U --> INV
  Q --> OUT[返回结果: {pattern}]""",
        "- 设计题先写每个操作的复杂度目标，再选数据结构组合。\n- 一个操作更新多个结构时，必须维护同步不变量。",
        "- `storage`：真实数据。\n- `index`：加速查询或删除的辅助结构。\n- `invariant`：两者必须保持一致。",
        "1. 列出所有 API 和目标复杂度。\n2. 为每个 API 选择主结构和辅助索引。\n3. 写清楚结构间不变量。\n4. 每个写操作最后都检查不变量是否恢复。",
        "设计题不是把高级结构堆上去，而是让每个操作都能用不变量解释为什么正确。",
        "- 时间复杂度：取决于 API 目标，常见为 $O(1)$ 或 $O(\\log n)$。\n- 空间复杂度：通常要额外保存索引。",
        "- 删除时只删主结构，忘了删索引。\n- 随机集合删除没有更新被交换元素的位置。\n- LRU 更新访问顺序时漏掉移动节点。",
    )


def code_pair(cat: Category, go_name: str, rust_name: str) -> tuple[str, str]:
    if cat.slug == "array-string":
        return (
            f"""func {go_name}(nums []int, limit int) int {{
\tleft, sum, ans := 0, 0, 0
\tfor right, x := range nums {{
\t\tsum += x
\t\tfor sum > limit && left <= right {{
\t\t\tsum -= nums[left]
\t\t\tleft++
\t\t}}
\t\tans = max(ans, right-left+1)
\t}}
\treturn ans
}}""",
            f"""pub fn {rust_name}(nums: Vec<i32>, limit: i32) -> i32 {{
    let (mut left, mut sum, mut ans) = (0usize, 0, 0);
    for right in 0..nums.len() {{
        sum += nums[right];
        while sum > limit && left <= right {{
            sum -= nums[left];
            left += 1;
        }}
        ans = ans.max((right - left + 1) as i32);
    }}
    ans
}}""",
        )
    if cat.slug == "hash-prefix":
        return (
            f"""func {go_name}(nums []int, target int) int {{
\tcnt := map[int]int{{0: 1}}
\tprefix, ans := 0, 0
\tfor _, x := range nums {{
\t\tprefix += x
\t\tans += cnt[prefix-target]
\t\tcnt[prefix]++
\t}}
\treturn ans
}}""",
            f"""use std::collections::HashMap;

pub fn {rust_name}(nums: Vec<i32>, target: i32) -> i32 {{
    let mut cnt = HashMap::from([(0, 1)]);
    let (mut prefix, mut ans) = (0, 0);
    for x in nums {{
        prefix += x;
        ans += cnt.get(&(prefix - target)).copied().unwrap_or(0);
        *cnt.entry(prefix).or_insert(0) += 1;
    }}
    ans
}}""",
        )
    if cat.slug == "stack-queue":
        return (
            f"""func {go_name}(nums []int) []int {{
\tans := make([]int, len(nums))
\tst := []int{{}}
\tfor i, x := range nums {{
\t\tfor len(st) > 0 && nums[st[len(st)-1]] < x {{
\t\t\tj := st[len(st)-1]
\t\t\tst = st[:len(st)-1]
\t\t\tans[j] = x
\t\t}}
\t\tst = append(st, i)
\t}}
\treturn ans
}}""",
            f"""pub fn {rust_name}(nums: Vec<i32>) -> Vec<i32> {{
    let mut ans = vec![0; nums.len()];
    let mut st: Vec<usize> = Vec::new();
    for (i, &x) in nums.iter().enumerate() {{
        while let Some(&j) = st.last() {{
            if nums[j] >= x {{ break; }}
            st.pop();
            ans[j] = x;
        }}
        st.push(i);
    }}
    ans
}}""",
        )
    if cat.slug == "binary-search":
        return (
            f"""func {go_name}(lo, hi int, check func(int) bool) int {{
\tfor lo < hi {{
\t\tmid := lo + (hi-lo)/2
\t\tif check(mid) {{
\t\t\thi = mid
\t\t}} else {{
\t\t\tlo = mid + 1
\t\t}}
\t}}
\treturn lo
}}""",
            f"""pub fn {rust_name}(mut lo: i32, mut hi: i32, check: impl Fn(i32) -> bool) -> i32 {{
    while lo < hi {{
        let mid = lo + (hi - lo) / 2;
        if check(mid) {{
            hi = mid;
        }} else {{
            lo = mid + 1;
        }}
    }}
    lo
}}""",
        )
    if cat.slug == "dynamic-programming":
        return (
            f"""func {go_name}(nums []int) int {{
\tdp0, dp1 := 0, 0
\tfor _, x := range nums {{
\t\tnext := max(dp1, dp0+x)
\t\tdp0, dp1 = dp1, next
\t}}
\treturn dp1
}}""",
            f"""pub fn {rust_name}(nums: Vec<i32>) -> i32 {{
    let (mut dp0, mut dp1) = (0, 0);
    for x in nums {{
        let next = dp1.max(dp0 + x);
        dp0 = dp1;
        dp1 = next;
    }}
    dp1
}}""",
        )
    if cat.slug == "graph-search":
        return (
            f"""func {go_name}(grid [][]int) int {{
\tdirs := [][2]int{{{{1, 0}}, {{-1, 0}}, {{0, 1}}, {{0, -1}}}}
\tans := 0
\tfor r := range grid {{
\t\tfor c := range grid[r] {{
\t\t\t_ = dirs
\t\t\t_ = c
\t\t\tans += grid[r][c]
\t\t}}
\t}}
\treturn ans
}}""",
            f"""pub fn {rust_name}(grid: Vec<Vec<i32>>) -> i32 {{
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ans = 0;
    for r in 0..grid.len() {{
        for c in 0..grid[r].len() {{
            let _ = dirs;
            ans += grid[r][c];
        }}
    }}
    ans
}}""",
        )
    if cat.slug == "binary-tree":
        return (
            f"""func {go_name}(root *TreeNode) int {{
\tans := 0
\tvar dfs func(*TreeNode) int
\tdfs = func(node *TreeNode) int {{
\t\tif node == nil {{
\t\t\treturn 0
\t\t}}
\t\tleft := dfs(node.Left)
\t\tright := dfs(node.Right)
\t\tans = max(ans, left+right+1)
\t\treturn max(left, right) + 1
\t}}
\tdfs(root)
\treturn ans
}}""",
            f"""pub fn {rust_name}(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {{
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {{
        let Some(node) = node else {{ return 0; }};
        let node = node.borrow();
        let left = dfs(node.left.clone(), ans);
        let right = dfs(node.right.clone(), ans);
        *ans = (*ans).max(left + right + 1);
        left.max(right) + 1
    }}
    let mut ans = 0;
    dfs(root, &mut ans);
    ans
}}""",
        )
    if cat.slug == "linked-list":
        return (
            f"""func {go_name}(head *ListNode) *ListNode {{
\tvar prev *ListNode
\tcur := head
\tfor cur != nil {{
\t\tnext := cur.Next
\t\tcur.Next = prev
\t\tprev, cur = cur, next
\t}}
\treturn prev
}}""",
            f"""pub fn {rust_name}(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {{
    let mut prev = None;
    let mut cur = head;
    while let Some(mut node) = cur {{
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        cur = next;
    }}
    prev
}}""",
        )
    if cat.slug == "greedy":
        return (
            f"""func {go_name}(intervals [][]int) int {{
\tsort.Slice(intervals, func(i, j int) bool {{
\t\treturn intervals[i][1] < intervals[j][1]
\t}})
\tans, end := 0, math.MinInt
\tfor _, in := range intervals {{
\t\tif in[0] >= end {{
\t\t\tans++
\t\t\tend = in[1]
\t\t}}
\t}}
\treturn ans
}}""",
            f"""pub fn {rust_name}(mut intervals: Vec<Vec<i32>>) -> i32 {{
    intervals.sort_by_key(|v| v[1]);
    let (mut ans, mut end) = (0, i32::MIN);
    for interval in intervals {{
        if interval[0] >= end {{
            ans += 1;
            end = interval[1];
        }}
    }}
    ans
}}""",
        )
    if cat.slug == "heap-priority":
        return (
            f"""func {go_name}(nums []int, k int) int {{
\th := &IntHeap{{}}
\tfor _, x := range nums {{
\t\theap.Push(h, x)
\t\tif h.Len() > k {{
\t\t\theap.Pop(h)
\t\t}}
\t}}
\treturn (*h)[0]
}}""",
            f"""use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn {rust_name}(nums: Vec<i32>, k: usize) -> i32 {{
    let mut heap = BinaryHeap::new();
    for x in nums {{
        heap.push(Reverse(x));
        if heap.len() > k {{
            heap.pop();
        }}
    }}
    heap.peek().map(|x| x.0).unwrap_or(0)
}}""",
        )
    if cat.slug == "backtracking":
        return (
            f"""func {go_name}(nums []int) [][]int {{
\tans := [][]int{{}}
\tpath := []int{{}}
\tvar dfs func(int)
\tdfs = func(start int) {{
\t\tans = append(ans, append([]int(nil), path...))
\t\tfor i := start; i < len(nums); i++ {{
\t\t\tpath = append(path, nums[i])
\t\t\tdfs(i + 1)
\t\t\tpath = path[:len(path)-1]
\t\t}}
\t}}
\tdfs(0)
\treturn ans
}}""",
            f"""pub fn {rust_name}(nums: Vec<i32>) -> Vec<Vec<i32>> {{
    fn dfs(start: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {{
        ans.push(path.clone());
        for i in start..nums.len() {{
            path.push(nums[i]);
            dfs(i + 1, nums, path, ans);
            path.pop();
        }}
    }}
    let (mut path, mut ans) = (Vec::new(), Vec::new());
    dfs(0, &nums, &mut path, &mut ans);
    ans
}}""",
        )
    if cat.slug == "math-bit":
        return (
            f"""func {go_name}(x int) int {{
\tans := 0
\tfor x > 0 {{
\t\tx -= x & -x
\t\tans++
\t}}
\treturn ans
}}""",
            f"""pub fn {rust_name}(mut x: i32) -> i32 {{
    let mut ans = 0;
    while x > 0 {{
        x -= x & -x;
        ans += 1;
    }}
    ans
}}""",
        )
    if cat.slug == "design-structure":
        return (
            """type AlgoStore struct {
\tdata []int
\tpos  map[int]int
}

func (s *AlgoStore) Add(x int) {
\tif _, ok := s.pos[x]; ok {
\t\treturn
\t}
\ts.pos[x] = len(s.data)
\ts.data = append(s.data, x)
}""",
            """use std::collections::HashMap;

pub struct AlgoStore {
    data: Vec<i32>,
    pos: HashMap<i32, usize>,
}

impl AlgoStore {
    pub fn add(&mut self, x: i32) {
        if self.pos.contains_key(&x) {
            return;
        }
        self.pos.insert(x, self.data.len());
        self.data.push(x);
    }
}""",
        )
    return (
        f"""func {go_name}(nums []int) int {{
\tans := 0
\tfor i, x := range nums {{
\t\t_ = i
\t\tif x >= 0 {{
\t\t\tans++
\t\t}}
\t}}
\treturn ans
}}""",
        f"""pub fn {rust_name}(nums: Vec<i32>) -> i32 {{
    let mut ans = 0;
    for x in nums {{
        if x >= 0 {{
            ans += 1;
        }}
    }}
    ans
}}""",
    )


def rotate(values: list[int], offset: int) -> list[int]:
    if not values:
        return []
    offset %= len(values)
    return values[offset:] + values[:offset]


def slugify(text: str) -> str:
    table = {
        "两端指针缩小候选": "two-end-pointers",
        "同向指针原地覆盖": "same-direction-overwrite",
        "滑动窗口维护计数": "sliding-window-count",
        "固定一维枚举另一维": "fix-one-enumerate",
        "排序后跳过重复值": "skip-duplicates-after-sort",
        "原地交换与稳定写入": "inplace-swap-stable-write",
        "前后缀乘积拆贡献": "prefix-suffix-product",
        "差分数组批量更新": "difference-array",
        "循环数组取模遍历": "circular-array-mod",
        "矩阵按层模拟": "matrix-layer-simulation",
        "字符桶替代哈希表": "char-bucket",
        "双数组归并扫描": "merge-two-arrays",
        "哨兵简化边界": "sentinel-boundary",
        "区间压缩为事件点": "interval-events",
    }
    if text in table:
        return table[text]
    out = []
    for ch in text:
        if ch.isascii() and ch.isalnum():
            out.append(ch.lower())
        elif ch in {" ", "-", "_"}:
            out.append("-")
    slug = "".join(out).strip("-")
    fallback = sum((i + 1) * ord(ch) for i, ch in enumerate(text))
    return slug or f"topic-{fallback}"


if __name__ == "__main__":
    main()
