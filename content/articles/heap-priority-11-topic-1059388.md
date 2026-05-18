---
title: 任务冷却按剩余次数：堆与优先队列训练题解
category: 堆与优先队列
summary: 用最大剩余次数决定任务优先级，再用冷却队列控制重新入堆时机。
problem_ids: [621]
order: 111
---

# 任务冷却按剩余次数：堆与优先队列训练题解

任务调度器要求相同任务之间至少间隔 `n` 个单位时间。直觉上应优先安排剩余次数最多的任务，因为它最容易在后面造成冲突。

堆只解决“当前选谁”，冷却限制还需要一个队列记录“什么时候可以重新参与竞争”。

## 两个容器

- 大根堆 `ready`：保存当前可执行任务的剩余次数。
- 队列 `cooling`：保存刚执行过但还没冷却完的任务，元素是 `(readyTime, remaining)`。

每个时间点：

1. 先把冷却结束的任务从队列移回堆。
2. 如果堆非空，执行剩余次数最多的任务。
3. 执行后还有剩余，就放入冷却队列，解锁时间是 `time + n + 1`。
4. 如果堆为空但队列非空，当前时间是 idle。

## Go 模拟实现

```go
package main

import "container/heap"

type MaxHeap []int

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *MaxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

type Cooldown struct {
	readyTime int
	remaining int
}

func leastInterval(tasks []byte, n int) int {
	counts := [26]int{}
	for _, task := range tasks {
		counts[task-'A']++
	}

	ready := &MaxHeap{}
	heap.Init(ready)
	for _, c := range counts {
		if c > 0 {
			heap.Push(ready, c)
		}
	}

	time := 0
	queue := []Cooldown{}
	for ready.Len() > 0 || len(queue) > 0 {
		time++

		for len(queue) > 0 && queue[0].readyTime <= time {
			heap.Push(ready, queue[0].remaining)
			queue = queue[1:]
		}

		if ready.Len() == 0 {
			continue
		}

		remaining := heap.Pop(ready).(int) - 1
		if remaining > 0 {
			queue = append(queue, Cooldown{
				readyTime: time + n + 1,
				remaining: remaining,
			})
		}
	}

	return time
}
```

## 公式解和堆解的关系

这题有更简洁的公式解：看最高频任务的数量。但堆模拟更适合训练通用思路，尤其当题目要求输出具体调度序列，或者每个任务有不同冷却时间时，公式就不够用了。

公式解决的是计数结构；堆加冷却队列解决的是过程结构。

## 易错点

- 重新入堆时间是 `time + n + 1`，因为两个相同任务之间要隔 `n` 个单位。
- 每个时间点先释放冷却完成的任务，再选择执行任务。
- 如果堆为空不能直接结束，冷却队列里可能还有任务。

## 复杂度

设任务总数为 `m`，任务种类为 `c`。模拟时间包含 idle，堆操作是 `O(log c)`；在固定 26 种大写字母场景下可视为 `O(m)`。
