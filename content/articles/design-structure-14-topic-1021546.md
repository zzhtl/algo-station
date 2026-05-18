---
title: 快照数组版本列表：设计与数据结构训练题解
category: 设计与数据结构
summary: 每个下标单独保存版本变更，用 snap_id 加二分查询实现按版本读取。
problem_ids: [1146]
order: 114
---

# 快照数组版本列表：设计与数据结构训练题解

快照数组支持 `set(index,val)`、`snap()`、`get(index,snap_id)`。如果每次快照都复制整个数组，空间会变成 `O(length * snaps)`。更好的方式是只记录发生变化的位置。

## 状态设计

为每个下标保存版本列表：

```text
history[index] = [(snap_id, value), ...]
```

每次 `set` 只修改当前下标在当前 `snap_id` 下的值。`snap()` 返回当前版本号，然后版本号加一。`get` 在对应下标的历史里找最后一个 `snap_id <= target` 的值。

## Go 参考实现

```go
package main

type Version struct {
	id  int
	val int
}

type SnapshotArray struct {
	snapID  int
	history [][]Version
}

func Constructor(length int) SnapshotArray {
	history := make([][]Version, length)
	for i := range history {
		history[i] = []Version{{id: 0, val: 0}}
	}
	return SnapshotArray{history: history}
}

func (s *SnapshotArray) Set(index int, val int) {
	list := s.history[index]
	if list[len(list)-1].id == s.snapID {
		list[len(list)-1].val = val
	} else {
		list = append(list, Version{id: s.snapID, val: val})
	}
	s.history[index] = list
}

func (s *SnapshotArray) Snap() int {
	id := s.snapID
	s.snapID++
	return id
}

func (s *SnapshotArray) Get(index int, snapID int) int {
	list := s.history[index]
	left, right := 0, len(list)
	for left < right {
		mid := left + (right-left)/2
		if list[mid].id <= snapID {
			left = mid + 1
		} else {
			right = mid
		}
	}
	return list[left-1].val
}
```

## 为什么同一版本覆盖最后一条

在同一个 `snapID` 内多次 `set(index,val)`，快照前只有最后一次值有效。如果每次都 append，会制造冗余版本；覆盖最后一条可以保持历史更短，并且不改变查询结果。

初始化每个下标的历史为 `(0,0)`，可以保证 `get` 总能找到一个不大于目标版本的值，不需要额外处理空列表。

## 易错点

- `snap()` 返回的是旧的 `snapID`，然后再自增。
- `Get` 找的是最后一个 `id <= snapID`，不是等于。
- 同一版本多次设置要覆盖，避免历史列表膨胀。

## 复杂度

`Set` 摊还 `O(1)`，`Snap` 是 `O(1)`，`Get` 是 `O(log m)`，其中 `m` 是该下标的变更次数。空间与实际变更次数相关。
