---
title: 时间键值存储二分查找：设计与数据结构训练题解
category: 设计与数据结构
summary: 为每个 key 保存按时间递增的版本列表，用二分查找不超过目标时间的最新值。
problem_ids: [981]
order: 113
---

# 时间键值存储二分查找：设计与数据结构训练题解

时间键值存储支持：

- `set(key,value,timestamp)`：在某个时间写入值。
- `get(key,timestamp)`：查询不超过该时间的最新值。

由于同一个 key 的写入时间通常按递增顺序给出，可以为每个 key 保存版本列表，然后在列表里二分。

## 状态设计

```text
store[key] = [(timestamp1,value1), (timestamp2,value2), ...]
```

每个列表按 `timestamp` 递增。查询时要找最后一个 `timestamp <= target` 的版本。

## Go 参考实现

```go
package main

type Entry struct {
	time  int
	value string
}

type TimeMap struct {
	store map[string][]Entry
}

func Constructor() TimeMap {
	return TimeMap{store: map[string][]Entry{}}
}

func (tm *TimeMap) Set(key string, value string, timestamp int) {
	tm.store[key] = append(tm.store[key], Entry{time: timestamp, value: value})
}

func (tm *TimeMap) Get(key string, timestamp int) string {
	list := tm.store[key]
	left, right := 0, len(list)
	for left < right {
		mid := left + (right-left)/2
		if list[mid].time <= timestamp {
			left = mid + 1
		} else {
			right = mid
		}
	}
	if left == 0 {
		return ""
	}
	return list[left-1].value
}
```

## 二分含义

这份二分找的是第一个 `time > timestamp` 的位置 `left`。那么 `left-1` 就是最后一个不超过目标时间的版本。

这种“找右边界”的写法比手动保存答案更稳定，尤其适合时间版本查询和有序数组前驱查询。

## 易错点

- 没有任何版本不超过目标时间时返回空字符串。
- 二分区间使用 `[left,right)`，初始 `right = len(list)`。
- 如果题目不保证 set 时间递增，就必须插入时保持列表有序，或最后排序。

## 复杂度

`Set` 均摊 `O(1)`；`Get` 对该 key 的版本数做二分，时间 `O(log m)`；空间 `O(total sets)`。
