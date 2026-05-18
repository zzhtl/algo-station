---
title: 二分查找：把"找一个值"做对很容易，做精很难
category: 二分搜索
summary: 二分不只是在有序数组里找数。一旦你掌握"在答案上二分"的思路，最小化最大值、寻找峰值、第 k 大这些题全是同一招。
problem_ids: [704, 35, 34, 153, 162, 875, 410, 4]
order: 1
---

# 二分查找：把"找一个值"做对很容易，做精很难

## 标准模板（左闭右闭）

```rust
fn binary_search(nums: &[i32], target: i32) -> i32 {
    let (mut l, mut r) = (0i32, nums.len() as i32 - 1);
    while l <= r {                          // 注意 <=
        let mid = l + (r - l) / 2;          // 防溢出
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    -1
}
```

三个细节决定它能不能写对：

1. **区间约定**：左闭右闭 `[l, r]` → `while l <= r`，`r = mid - 1`。左闭右开 `[l, r)` → `while l < r`，`r = mid`。**不要混用**。
2. **`mid` 计算**：`l + (r - l) / 2` 避免 `l + r` 溢出。
3. **缩区间永远把 `mid` 那一格排除掉**，否则死循环。

## 找"左边界"和"右边界"

> 抽象问题：在升序数组中找目标值的**第一次出现**和**最后一次出现**位置。

**左边界**：找到目标后继续往左压。

```python
def lower_bound(nums, target):
    """返回第一个 >= target 的下标; 全都小于则返回 len(nums)"""
    l, r = 0, len(nums)              # 左闭右开
    while l < r:
        mid = (l + r) // 2
        if nums[mid] < target:
            l = mid + 1
        else:
            r = mid                  # 命中也继续向左
    return l
```

**右边界**：

```python
def upper_bound(nums, target):
    """返回第一个 > target 的下标"""
    l, r = 0, len(nums)
    while l < r:
        mid = (l + r) // 2
        if nums[mid] <= target:
            l = mid + 1
        else:
            r = mid
    return l
```

记住这两个函数（C++ STL 同名），各种"找区间"的题都能拼出来：

- 目标值数量 = `upper_bound(t) - lower_bound(t)`
- 严格小于 t 的元素数 = `lower_bound(t)`

## 在"答案"上二分（核心套路）

当你看到题目要求"**最小化最大值**"或"**最大化最小值**"，几乎一定是二分答案：

```mermaid
flowchart LR
    A["对答案值 x 建立单调判断 check(x)"] --> B["check 单调: x 满足 → x+1 也满足"]
    B --> C["在 x 的取值范围上二分"]
    C --> D["找到使 check 从 false 翻 true 的临界 x"]
```

模板：

```text
l, r = 答案下界, 答案上界
while l < r:
    mid = (l + r) // 2
    if check(mid):
        r = mid          // mid 可行,试更小
    else:
        l = mid + 1      // mid 不可行,必须更大
return l                 // 最小可行答案
```

### 例：爱吃香蕉的珂珂

> 抽象问题：有几堆香蕉，警卫还有 H 小时回来。每小时只能选一堆吃 K 根（不够就这一堆吃完）。求让她在 H 小时内吃完的**最小** K。

**check(k)**：以速度 k 吃，总共要多少小时？$\sum \lceil piles_i / k \rceil$。

```rust
fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut l, mut r) = (1i64, *piles.iter().max().unwrap() as i64);
    let check = |k: i64| -> bool {
        let mut hours = 0i64;
        for &p in &piles {
            hours += (p as i64 + k - 1) / k;          // 向上取整
        }
        hours <= h as i64
    };
    while l < r {
        let mid = l + (r - l) / 2;
        if check(mid) { r = mid; } else { l = mid + 1; }
    }
    l as i32
}
```

### 例：分割数组的最大值

> 抽象问题：把数组分成 k 个连续子数组，使**子数组和的最大值**最小，求这个最小化的最大值。

**check(x)**：能否在"每段和 ≤ x"的约束下用 ≤ k 段切完？贪心扫一遍即可。

二分的答案范围：`max(nums)` 到 `sum(nums)`。

## 旋转排序数组中查找

> 抽象问题：升序数组在某点旋转后（如 `[4,5,6,7,0,1,2]`），找目标值的位置。

观察：**任意 mid 切一刀，左右两半至少有一边是单调的**。

```python
def search_rotated(nums, target):
    l, r = 0, len(nums) - 1
    while l <= r:
        mid = (l + r) // 2
        if nums[mid] == target: return mid
        if nums[l] <= nums[mid]:                # 左半单调
            if nums[l] <= target < nums[mid]:
                r = mid - 1
            else:
                l = mid + 1
        else:                                   # 右半单调
            if nums[mid] < target <= nums[r]:
                l = mid + 1
            else:
                r = mid - 1
    return -1
```

## 寻找峰值（无序也能二分）

> 抽象问题：数组中峰值定义为大于左右邻居的元素，找任一峰值的下标。可假设 `nums[-1] = nums[n] = -∞`。

**单调性来源**：如果 `nums[mid] < nums[mid+1]`，右边一定存在峰值（沿着升的方向走必然撞到顶）。

```go
func findPeak(nums []int) int {
    l, r := 0, len(nums)-1
    for l < r {
        mid := (l + r) / 2
        if nums[mid] < nums[mid+1] {
            l = mid + 1
        } else {
            r = mid
        }
    }
    return l
}
```

## 二分能用的判别

满足下面**任一条**：

1. 序列已**全局排序**（找元素或边界）。
2. 序列**局部单调**且条件能区分两侧（如旋转数组、峰值）。
3. **答案空间单调**：`check(x)` 是单调函数 → 在 x 上二分。

## 让人崩溃的死循环

死循环 99% 是缩区间没把 `mid` 排除掉。两个典型错误：

```text
# ❌ 左闭右闭区间忘了 ±1
while l < r:
    mid = (l + r) // 2
    if check(mid): r = mid
    else: l = mid           # 错！应为 l = mid + 1

# ❌ 上取整 mid 才能用 r = mid - 1
while l < r:
    mid = (l + r + 1) // 2  # 偏右的 mid
    if check(mid): l = mid
    else: r = mid - 1
```

**口诀**：

- `l = mid`：用偏右 mid `(l+r+1)/2`
- `r = mid`：用偏左 mid `(l+r)/2`
- `l = mid+1` / `r = mid-1`：偏左偏右都行

## 常见坑速查

| 坑 | 修复 |
| --- | --- |
| 区间约定混用 | 全程统一闭/开区间 |
| `(l+r)/2` 大数溢出 | 用 `l + (r-l)/2` |
| 缩区间没排除 mid | 死循环 |
| 找不到时返回值错误 | 想清楚 `lower_bound` 返回的是"插入位置" |
| 升降序判别错 | 一定先看清是升序还是降序 |

## 相关题目

- #704 二分查找（基础）
- #35 搜索插入位置（lower_bound）
- #34 在排序数组中查找元素的第一个和最后一个位置（双边界）
- #153 寻找旋转排序数组中的最小值
- #162 寻找峰值
- #875 爱吃香蕉的珂珂（答案二分）
- #410 分割数组的最大值（答案二分 + 贪心 check）
- #4 寻找两个正序数组的中位数（分治式二分，难）
