export type VisualKind = 'array' | 'linked_list' | 'stack' | 'heap' | 'tree' | 'trie' | 'graph' | 'table' | 'string';
export type CellState = 'normal' | 'active' | 'candidate' | 'done' | 'pivot' | 'muted';

export interface VisualCell {
  label: string;
  value: string;
  state: CellState;
}

export interface VisualPointer {
  label: string;
  index: number;
}

export interface VisualNode {
  id: string;
  label: string;
  x: number;
  y: number;
  state: CellState;
}

export interface VisualEdge {
  from: string;
  to: string;
  label?: string;
  state: CellState;
}

export interface VisualTable {
  columns: string[];
  rows: { label: string; values: string[] }[];
  active: [number, number][];
}

export interface VisualFrame {
  title: string;
  description: string;
  codeLine: string;
  cells?: VisualCell[];
  pointers?: VisualPointer[];
  nodes?: VisualNode[];
  edges?: VisualEdge[];
  table?: VisualTable;
  stack?: string[];
  takeaway?: string;
}

export interface VisualizationSpec {
  id: string;
  title: string;
  kind: VisualKind;
  frames: VisualFrame[];
}

type ArrayStep = {
  title: string;
  description: string;
  values: (string | number)[];
  active?: number[];
  candidate?: number[];
  done?: number[];
  pivot?: number[];
  pointers?: VisualPointer[];
  stack?: string[];
  code: string;
  takeaway?: string;
};

function arraySpec(id: string, title: string, kind: VisualKind, steps: ArrayStep[]): VisualizationSpec {
  return {
    id,
    title,
    kind,
    frames: steps.map((step) => ({
      title: step.title,
      description: step.description,
      codeLine: step.code,
      cells: step.values.map((value, index) => ({
        label: String(index),
        value: String(value),
        state: step.pivot?.includes(index)
          ? 'pivot'
          : step.active?.includes(index)
            ? 'active'
            : step.candidate?.includes(index)
              ? 'candidate'
              : step.done?.includes(index)
                ? 'done'
                : 'normal'
      })),
      pointers: step.pointers,
      stack: step.stack,
      takeaway: step.takeaway
    }))
  };
}

const treeNodes = (states: Record<string, CellState> = {}): VisualNode[] => [
  { id: '4', label: '4', x: 50, y: 12, state: states['4'] ?? 'normal' },
  { id: '2', label: '2', x: 27, y: 42, state: states['2'] ?? 'normal' },
  { id: '6', label: '6', x: 73, y: 42, state: states['6'] ?? 'normal' },
  { id: '1', label: '1', x: 14, y: 76, state: states['1'] ?? 'normal' },
  { id: '3', label: '3', x: 39, y: 76, state: states['3'] ?? 'normal' },
  { id: '5', label: '5', x: 61, y: 76, state: states['5'] ?? 'normal' },
  { id: '7', label: '7', x: 86, y: 76, state: states['7'] ?? 'normal' }
];

const treeEdges: VisualEdge[] = [
  ['4', '2'], ['4', '6'], ['2', '1'], ['2', '3'], ['6', '5'], ['6', '7']
].map(([from, to]) => ({ from, to, state: 'normal' }));

const graphNodes = (states: Record<string, CellState> = {}): VisualNode[] => [
  { id: 'A', label: 'A', x: 16, y: 25, state: states.A ?? 'normal' },
  { id: 'B', label: 'B', x: 42, y: 12, state: states.B ?? 'normal' },
  { id: 'C', label: 'C', x: 72, y: 24, state: states.C ?? 'normal' },
  { id: 'D', label: 'D', x: 28, y: 72, state: states.D ?? 'normal' },
  { id: 'E', label: 'E', x: 63, y: 72, state: states.E ?? 'normal' }
];

const graphEdges: VisualEdge[] = [
  ['A', 'B'], ['A', 'D'], ['B', 'C'], ['B', 'D'], ['C', 'E'], ['D', 'E']
].map(([from, to]) => ({ from, to, state: 'normal' }));

function nodeFrame(
  title: string,
  description: string,
  codeLine: string,
  nodes: VisualNode[],
  edges: VisualEdge[],
  stack: string[],
  takeaway?: string
): VisualFrame {
  return { title, description, codeLine, nodes, edges, stack, takeaway };
}

export const visualizationSpecs: VisualizationSpec[] = [
  arraySpec('insertion-sort', '插入排序', 'array', [
    { title: '有序区只有一个元素', description: '左侧 [5] 天然有序，key 指向 2。', values: [5, 2, 4, 6, 1], done: [0], active: [1], pointers: [{ label: 'key', index: 1 }], code: 'for i = 1..n-1' },
    { title: '较大元素右移', description: '5 比 key=2 大，先把 5 向右挪一格。', values: [5, 5, 4, 6, 1], active: [0, 1], pointers: [{ label: 'j', index: 0 }], code: 'while j >= 0 && a[j] > key' },
    { title: '把 key 插入空位', description: '2 放到下标 0，有序区扩展为 [2,5]。', values: [2, 5, 4, 6, 1], done: [0, 1], pointers: [{ label: 'key', index: 0 }], code: 'a[j + 1] = key' },
    { title: '继续扩展有序区', description: '4 插入 2 与 5 之间，6 无需移动。', values: [2, 4, 5, 6, 1], done: [0, 1, 2, 3], active: [1], code: '每轮结束：a[0..i] 有序' },
    { title: '最终插入 1', description: '前四个元素都右移，1 进入开头。', values: [1, 2, 4, 5, 6], done: [0, 1, 2, 3, 4], code: '循环不变量得到全局有序', takeaway: '记忆点：不是交换 key，而是先“挪出空位”再插入。' }
  ]),
  arraySpec('merge-sort', '归并排序', 'array', [
    { title: '递归拆成两半', description: '[7,2,5,3] 拆成两个更小的排序任务。', values: [7, 2, 5, 3], active: [0, 1], candidate: [2, 3], code: 'sort(l, mid); sort(mid + 1, r)', stack: ['sort(0,3)', 'sort(0,1)'] },
    { title: '子数组已有序', description: '递归返回后，左右两段分别为 [2,7] 与 [3,5]。', values: [2, 7, 3, 5], done: [0, 1], candidate: [2, 3], code: 'merge([2,7], [3,5])' },
    { title: '双指针取较小值', description: '2 先进入临时数组，然后比较 7 与 3。', values: [2, 7, 3, 5], active: [0, 2], pointers: [{ label: 'i', index: 0 }, { label: 'j', index: 2 }], stack: ['tmp = [2]'], code: 'tmp.push(min(left[i], right[j]))' },
    { title: '归并完成', description: '按 2、3、5、7 的顺序写回原数组。', values: [2, 3, 5, 7], done: [0, 1, 2, 3], code: 'copy tmp back to a[l..r]', takeaway: '记忆点：递归负责“变有序”，双指针负责“合起来”。' }
  ]),
  arraySpec('quick-sort-partition', '快速排序划分', 'array', [
    { title: '选择末尾为 pivot', description: 'pivot=4，i 表示“小于等于 pivot 区”的右边界。', values: [3, 7, 2, 5, 4], pivot: [4], pointers: [{ label: 'i', index: 0 }, { label: 'j', index: 0 }], code: 'pivot = a[r]; i = l' },
    { title: '跳过较大元素', description: '3 已进入小区；7 大于 4，只移动扫描指针 j。', values: [3, 7, 2, 5, 4], done: [0], active: [1], pivot: [4], pointers: [{ label: 'i', index: 1 }, { label: 'j', index: 1 }], code: 'if a[j] <= pivot' },
    { title: '把 2 换进小区', description: '交换 a[i]=7 与 a[j]=2，小区连续扩张。', values: [3, 2, 7, 5, 4], done: [0, 1], active: [2], pivot: [4], pointers: [{ label: 'i', index: 2 }, { label: 'j', index: 2 }], code: 'swap(a[i], a[j]); i++' },
    { title: 'pivot 就位', description: '扫描结束后交换 pivot 与 a[i]，左右满足大小关系。', values: [3, 2, 4, 5, 7], done: [2], pivot: [2], code: 'swap(a[i], a[r])', takeaway: '记忆点：i 永远指向“下一个应该放小元素的位置”。' }
  ]),
  arraySpec('two-pointers', '相向双指针', 'array', [
    { title: '从两端开始', description: '有序数组中寻找和为 10 的一对数。', values: [1, 2, 4, 6, 9], active: [0, 4], pointers: [{ label: 'L', index: 0 }, { label: 'R', index: 4 }], code: 'while L < R' },
    { title: '当前和正好命中', description: '1 + 9 = 10，找到答案；若目标是 8，则会因为和偏大而左移 R。', values: [1, 2, 4, 6, 9], done: [0, 4], pointers: [{ label: 'L', index: 0 }, { label: 'R', index: 4 }], code: 'sum = a[L] + a[R]' },
    { title: '和偏小就右移 L', description: '目标换成 11：1+9=10 偏小，只有增大左值才可能命中。', values: [1, 2, 4, 6, 9], active: [1, 4], pointers: [{ label: 'L', index: 1 }, { label: 'R', index: 4 }], code: 'if sum < target { L++ }' },
    { title: '搜索空间单调缩小', description: '2+9=11 命中；每次排除一整行不可能组合。', values: [1, 2, 4, 6, 9], done: [1, 4], code: '每步至少移动一个指针', takeaway: '记忆点：移动哪边取决于“怎样让当前值朝目标变化”。' }
  ]),
  arraySpec('sliding-window', '滑动窗口', 'array', [
    { title: '右端扩张', description: '寻找和至少为 7 的最短子数组，先不断加入右端元素。', values: [2, 3, 1, 2, 4, 3], active: [0, 1, 2, 3], pointers: [{ label: 'L', index: 0 }, { label: 'R', index: 3 }], code: 'sum += a[R]' },
    { title: '满足条件后收缩', description: '窗口和为 8，记录长度 4，再尝试移走左端 2。', values: [2, 3, 1, 2, 4, 3], candidate: [0, 1, 2, 3], pointers: [{ label: 'L', index: 0 }, { label: 'R', index: 3 }], code: 'while sum >= target' },
    { title: '继续扩张再收缩', description: '加入 4 后，从左侧连续移除 3、1，得到窗口 [2,4]。', values: [2, 3, 1, 2, 4, 3], active: [3, 4], done: [0, 1, 2], pointers: [{ label: 'L', index: 3 }, { label: 'R', index: 4 }], code: 'answer = min(answer, R-L+1)' },
    { title: '得到最短窗口', description: '末尾 [4,3] 长度为 2，无法再缩短。', values: [2, 3, 1, 2, 4, 3], done: [4, 5], code: '左端只前进，整体 O(n)', takeaway: '记忆点：右端负责获得可行性，左端负责压缩冗余。' }
  ]),
  arraySpec('prefix-difference', '前缀和与差分', 'array', [
    { title: '原数组', description: '先观察原值 [2,1,3,2]。', values: [2, 1, 3, 2], active: [0], code: 'prefix[0] = 0' },
    { title: '累加得到前缀和', description: 'prefix[i+1] 保存前 i+1 个数的总和。', values: [0, 2, 3, 6, 8], done: [0, 1, 2, 3, 4], code: 'prefix[i+1] = prefix[i] + a[i]' },
    { title: '区间和变成相减', description: 'a[1..3] 的和 = prefix[4] - prefix[1] = 6。', values: [0, 2, 3, 6, 8], active: [1, 4], pointers: [{ label: 'l', index: 1 }, { label: 'r+1', index: 4 }], code: 'sum(l,r) = prefix[r+1] - prefix[l]' },
    { title: '差分做区间修改', description: '区间 [1,3] 加 2，只在边界做 +2 与 -2，最后累加还原。', values: [0, 2, 0, 0, -2], active: [1, 4], code: 'diff[l] += x; diff[r+1] -= x', takeaway: '记忆点：前缀把区间查询变相减；差分把区间修改变边界标记。' }
  ]),
  arraySpec('linked-list-reverse', '链表反转', 'linked_list', [
    { title: '三指针准备', description: 'prev 为空，cur 指向 1，先保存 next=2。', values: ['∅', 1, 2, 3], active: [1, 2], pointers: [{ label: 'prev', index: 0 }, { label: 'cur', index: 1 }, { label: 'next', index: 2 }], code: 'next = cur.next' },
    { title: '反转当前箭头', description: '把 1.next 指向 prev，链表暂时分成两段。', values: ['∅', 1, 2, 3], done: [1], active: [2], pointers: [{ label: 'prev', index: 1 }, { label: 'cur', index: 2 }], code: 'cur.next = prev' },
    { title: '整体向前推进', description: '保存 3，再让 2 指向 1。', values: ['∅', 1, 2, 3], done: [1, 2], active: [3], pointers: [{ label: 'prev', index: 2 }, { label: 'cur', index: 3 }], code: 'prev = cur; cur = next' },
    { title: 'cur 走到空', description: '全部箭头反向，prev=3 成为新头结点。', values: [3, 2, 1, '∅'], done: [0, 1, 2], pointers: [{ label: 'head', index: 0 }], code: 'return prev', takeaway: '记忆点：先存 next，再改箭头；顺序反了就会丢链。' }
  ]),
  arraySpec('fast-slow-cycle', '快慢指针判环', 'linked_list', [
    { title: '同一起点', description: 'slow 每次一步，fast 每次两步。', values: [1, 2, 3, 4, 5, '↩2'], active: [0], pointers: [{ label: 'slow', index: 0 }, { label: 'fast', index: 0 }], code: 'slow = fast = head' },
    { title: '速度差逐步追近', description: 'slow 到 2，fast 到 3。', values: [1, 2, 3, 4, 5, '↩2'], active: [1, 2], pointers: [{ label: 'slow', index: 1 }, { label: 'fast', index: 2 }], code: 'slow = slow.next; fast = fast.next.next' },
    { title: '都进入环中', description: 'slow 到 3，fast 到 5；环内相对距离每轮减 1。', values: [1, 2, 3, 4, 5, '↩2'], candidate: [2, 4], pointers: [{ label: 'slow', index: 2 }, { label: 'fast', index: 4 }], code: 'if fast == nil return false' },
    { title: '指针相遇', description: 'fast 绕回后与 slow 相遇，证明存在环。', values: [1, 2, 3, 4, 5, '↩2'], done: [3], pointers: [{ label: 'slow=fast', index: 3 }], code: 'if slow == fast return true', takeaway: '记忆点：环内快指针每轮相对多走一步，必然追上。' }
  ]),
  arraySpec('monotonic-stack', '单调栈', 'stack', [
    { title: '维护递增下标栈', description: '柱高 2 入栈，栈内从底到顶递增。', values: [2, 1, 5, 6, 2, 3], active: [0], stack: ['0(高2)'], code: 'while stack && h[top] > h[i]' },
    { title: '遇到更矮元素弹栈', description: '高度 1 使高度 2 弹出，此刻确定 2 的右边界。', values: [2, 1, 5, 6, 2, 3], done: [0], active: [1], stack: ['1(高1)'], code: 'top = stack.pop()' },
    { title: '高柱连续入栈', description: '5、6 递增，暂时不知道它们的右边界。', values: [2, 1, 5, 6, 2, 3], active: [2, 3], stack: ['1(高1)', '2(高5)', '3(高6)'], code: 'stack.push(i)' },
    { title: '高度 2 触发结算', description: '先弹 6 再弹 5，弹出时两侧第一个更矮位置都已知。', values: [2, 1, 5, 6, 2, 3], done: [2, 3], active: [4], stack: ['1(高1)', '4(高2)'], code: 'width = i - stack.top - 1', takeaway: '记忆点：元素出栈的那一刻，正是答案边界信息齐全的时刻。' }
  ]),
  arraySpec('binary-heap', '二叉堆插入与删除', 'heap', [
    { title: '最小堆数组表示', description: '父子关系由下标计算，不需要显式指针。', values: [2, 5, 4, 9, 7], done: [0], code: 'parent(i) = (i-1)/2' },
    { title: '新元素放到末尾', description: '插入 1，先保持完全二叉树形状。', values: [2, 5, 4, 9, 7, 1], active: [5], pointers: [{ label: 'i', index: 5 }], code: 'heap.push(1)' },
    { title: '向上调整', description: '1 依次与父结点 4、2 交换。', values: [1, 5, 2, 9, 7, 4], active: [0], done: [2, 5], code: 'while heap[parent] > heap[i] swap' },
    { title: '删除堆顶后下沉', description: '用末尾 4 覆盖堆顶，再与更小的孩子 2 交换。', values: [2, 5, 4, 9, 7], done: [0], active: [2], code: 'swap with smaller child', takeaway: '记忆点：插入先放末尾再上浮；删除先末尾补顶再下沉。' }
  ]),
  arraySpec('binary-search-boundary', '二分边界', 'array', [
    { title: '定义左闭右开区间', description: '在 [1,2,2,2,5] 中找第一个 ≥2 的位置。', values: [1, 2, 2, 2, 5], active: [0, 1, 2, 3, 4], pointers: [{ label: 'L', index: 0 }, { label: 'R', index: 4 }], code: 'L = 0; R = n' },
    { title: 'mid 满足条件', description: 'mid=2 的值为 2，答案可能是它或更左边，所以收缩 R。', values: [1, 2, 2, 2, 5], candidate: [2], pointers: [{ label: 'mid', index: 2 }], code: 'if a[mid] >= target { R = mid }' },
    { title: 'mid 不满足条件', description: '区间变为 [0,2)，mid=1? 先检查 mid=1 命中；再检查 0 不满足。', values: [1, 2, 2, 2, 5], active: [0, 1], pointers: [{ label: 'mid', index: 0 }], code: 'else { L = mid + 1 }' },
    { title: 'L 与 R 相遇', description: '相遇点 1 就是第一个满足谓词的位置。', values: [1, 2, 2, 2, 5], done: [1], pointers: [{ label: 'L=R', index: 1 }], code: 'return L', takeaway: '记忆点：先写“满足条件时保留哪一半”，边界自然跟着区间定义走。' }
  ]),
  arraySpec('binary-search-answer', '答案二分', 'array', [
    { title: '答案具有单调性', description: '运货能力越大，所需天数只会不变或减少。', values: [10, 15, 20, 25, 30], active: [0, 1, 2, 3, 4], code: 'predicate(capacity) = days <= D' },
    { title: '检查中间答案', description: 'mid=20 可在 D 天内完成，因此答案不必更大。', values: [10, 15, 20, 25, 30], candidate: [2], done: [3, 4], pointers: [{ label: 'mid', index: 2 }], code: 'if feasible(mid) { R = mid }' },
    { title: '排除不可行区', description: 'mid=15 不可行，所有更小容量也不可行。', values: [10, 15, 20, 25, 30], active: [1], done: [0], code: 'else { L = mid + 1 }' },
    { title: '找到最小可行值', description: '边界停在 20，左侧不可行、右侧可行。', values: [10, 15, 20, 25, 30], done: [2], code: 'return first feasible', takeaway: '记忆点：题目没给有序数组，但“答案是否可行”构成了布尔有序数组。' }
  ]),
  {
    id: 'tree-dfs', title: '二叉树深度优先遍历', kind: 'tree', frames: [
      nodeFrame('从根进入', '前序 DFS 先处理当前结点 4，再递归左子树。', 'visit(node); dfs(node.left)', treeNodes({ '4': 'active' }), treeEdges, ['dfs(4)']),
      nodeFrame('沿左链深入', '调用栈保存尚未完成的父结点。', 'dfs(2); dfs(1)', treeNodes({ '4': 'done', '2': 'active', '1': 'candidate' }), treeEdges, ['dfs(4)', 'dfs(2)', 'dfs(1)']),
      nodeFrame('回溯并转向右支', '1 的孩子为空，返回 2 后访问 3。', 'return; dfs(node.right)', treeNodes({ '4': 'done', '2': 'done', '1': 'done', '3': 'active' }), treeEdges, ['dfs(4)', 'dfs(2)', 'dfs(3)']),
      nodeFrame('遍历完成', '每个结点恰好进入和离开一次，前序为 4,2,1,3,6,5,7。', '递归返回值向父层汇总', treeNodes({ '4': 'done', '2': 'done', '6': 'done', '1': 'done', '3': 'done', '5': 'done', '7': 'done' }), treeEdges, ['4 → 2 → 1 → 3 → 6 → 5 → 7'], '记忆点：递归函数要说清“当前结点负责什么、向父层返回什么”。')
    ]
  },
  {
    id: 'tree-bfs', title: '二叉树层序遍历', kind: 'tree', frames: [
      nodeFrame('根结点入队', '队列保存下一批等待访问的结点。', 'queue.push(root)', treeNodes({ '4': 'active' }), treeEdges, ['队列: 4']),
      nodeFrame('弹出一整层', '读取当前队列长度 1，处理 4 并加入 2、6。', 'size = queue.length', treeNodes({ '4': 'done', '2': 'candidate', '6': 'candidate' }), treeEdges, ['本层: 4', '队列: 2, 6']),
      nodeFrame('固定长度避免串层', '本层只处理最初的 2 个结点，新加入的孩子留给下一轮。', 'repeat size times', treeNodes({ '4': 'done', '2': 'active', '6': 'active', '1': 'candidate', '3': 'candidate', '5': 'candidate', '7': 'candidate' }), treeEdges, ['本层: 2, 6', '队列: 1, 3, 5, 7']),
      nodeFrame('按层完成', '层序结果为 [4]、[2,6]、[1,3,5,7]。', 'level++', treeNodes({ '4': 'done', '2': 'done', '6': 'done', '1': 'done', '3': 'done', '5': 'done', '7': 'done' }), treeEdges, ['[4]', '[2, 6]', '[1, 3, 5, 7]'], '记忆点：每轮先锁定 queue.length，它就是这一层的边界。')
    ]
  },
  {
    id: 'trie', title: 'Trie 插入与查找', kind: 'trie', frames: [
      nodeFrame('从根读取字符', '插入 cat，从根沿字符 c 查找孩子。', 'node = root', [
        { id: 'root', label: 'root', x: 50, y: 10, state: 'active' }, { id: 'c', label: 'c', x: 32, y: 40, state: 'candidate' }, { id: 'd', label: 'd', x: 68, y: 40, state: 'normal' }, { id: 'a', label: 'a', x: 32, y: 68, state: 'normal' }, { id: 't', label: 't', x: 32, y: 91, state: 'normal' }
      ], [{ from: 'root', to: 'c', state: 'normal' }, { from: 'root', to: 'd', state: 'normal' }, { from: 'c', to: 'a', state: 'normal' }, { from: 'a', to: 't', state: 'normal' }], ['word = "cat"']),
      nodeFrame('逐字符向下', 'c 已存在，再沿 a、t 前进；缺失时才创建结点。', 'node = node.children[ch]', [
        { id: 'root', label: 'root', x: 50, y: 10, state: 'done' }, { id: 'c', label: 'c', x: 32, y: 40, state: 'done' }, { id: 'd', label: 'd', x: 68, y: 40, state: 'normal' }, { id: 'a', label: 'a', x: 32, y: 68, state: 'active' }, { id: 't', label: 't', x: 32, y: 91, state: 'candidate' }
      ], [{ from: 'root', to: 'c', state: 'done' }, { from: 'root', to: 'd', state: 'normal' }, { from: 'c', to: 'a', state: 'done' }, { from: 'a', to: 't', state: 'active' }], ['c → a → t']),
      nodeFrame('标记单词结尾', '到达 t 后设置 isEnd=true，区分 cat 与前缀 ca。', 'node.isEnd = true', [
        { id: 'root', label: 'root', x: 50, y: 10, state: 'normal' }, { id: 'c', label: 'c', x: 32, y: 40, state: 'done' }, { id: 'd', label: 'd', x: 68, y: 40, state: 'normal' }, { id: 'a', label: 'a', x: 32, y: 68, state: 'done' }, { id: 't', label: 't ✓', x: 32, y: 91, state: 'done' }
      ], [{ from: 'root', to: 'c', state: 'done' }, { from: 'root', to: 'd', state: 'normal' }, { from: 'c', to: 'a', state: 'done' }, { from: 'a', to: 't', state: 'done' }], ['isEnd(t) = true']),
      nodeFrame('查找复用同一路径', 'search("ca") 路径存在但末尾未标记；startsWith("ca") 则成功。', 'search 检查 isEnd；前缀查询不检查', [
        { id: 'root', label: 'root', x: 50, y: 10, state: 'normal' }, { id: 'c', label: 'c', x: 32, y: 40, state: 'done' }, { id: 'd', label: 'd', x: 68, y: 40, state: 'normal' }, { id: 'a', label: 'a', x: 32, y: 68, state: 'active' }, { id: 't', label: 't ✓', x: 32, y: 91, state: 'normal' }
      ], [{ from: 'root', to: 'c', state: 'done' }, { from: 'root', to: 'd', state: 'normal' }, { from: 'c', to: 'a', state: 'done' }, { from: 'a', to: 't', state: 'normal' }], ['search(ca)=false', 'startsWith(ca)=true'], '记忆点：路径表示前缀，isEnd 才表示完整单词。')
    ]
  },
  {
    id: 'backtracking-tree', title: '回溯决策树', kind: 'tree', frames: [
      nodeFrame('从空路径出发', '生成 [1,2,3] 的排列，根结点表示 path=[]。', 'backtrack(path, used)', [
        { id: 'r', label: '[]', x: 50, y: 10, state: 'active' }, { id: '1', label: '[1]', x: 20, y: 42, state: 'normal' }, { id: '2', label: '[2]', x: 50, y: 42, state: 'normal' }, { id: '3', label: '[3]', x: 80, y: 42, state: 'normal' }
      ], [{ from: 'r', to: '1', state: 'normal' }, { from: 'r', to: '2', state: 'normal' }, { from: 'r', to: '3', state: 'normal' }], ['path=[]']),
      nodeFrame('做出选择', '选择 1 后标记 used[1]，下一层只能选 2 或 3。', 'path.push(1); used[1] = true', [
        { id: 'r', label: '[]', x: 50, y: 10, state: 'done' }, { id: '1', label: '[1]', x: 20, y: 42, state: 'active' }, { id: '12', label: '[1,2]', x: 10, y: 75, state: 'candidate' }, { id: '13', label: '[1,3]', x: 30, y: 75, state: 'normal' }
      ], [{ from: 'r', to: '1', state: 'done' }, { from: '1', to: '12', state: 'active' }, { from: '1', to: '13', state: 'normal' }], ['path=[1]', 'used={1}']),
      nodeFrame('到叶子收集答案', '[1,2,3] 长度达到 n，复制到结果。', 'result.push(copy(path))', [
        { id: 'r', label: '[]', x: 50, y: 8, state: 'done' }, { id: '1', label: '[1]', x: 20, y: 34, state: 'done' }, { id: '12', label: '[1,2]', x: 14, y: 62, state: 'done' }, { id: '123', label: '[1,2,3] ✓', x: 14, y: 90, state: 'active' }
      ], [{ from: 'r', to: '1', state: 'done' }, { from: '1', to: '12', state: 'done' }, { from: '12', to: '123', state: 'active' }], ['答案 +1']),
      nodeFrame('撤销后探索兄弟分支', '弹出 3、取消标记，再回到 [1] 选择 3。', 'used[x] = false; path.pop()', [
        { id: 'r', label: '[]', x: 50, y: 10, state: 'done' }, { id: '1', label: '[1]', x: 20, y: 42, state: 'active' }, { id: '12', label: '[1,2]', x: 10, y: 75, state: 'done' }, { id: '13', label: '[1,3]', x: 30, y: 75, state: 'candidate' }
      ], [{ from: 'r', to: '1', state: 'done' }, { from: '1', to: '12', state: 'done' }, { from: '1', to: '13', state: 'active' }], ['path 回到 [1]'], '记忆点：选择、递归、撤销必须成对出现，路径代表当前递归栈。')
    ]
  },
  {
    id: 'graph-dfs', title: '图的深度优先搜索', kind: 'graph', frames: [
      nodeFrame('从 A 开始', '先标记 visited，避免无向边把搜索带回原点。', 'visited.add(A)', graphNodes({ A: 'active' }), graphEdges, ['dfs(A)']),
      nodeFrame('沿一条边深入', '选择邻居 B，再进入 C。', 'for next in graph[node]', graphNodes({ A: 'done', B: 'done', C: 'active' }), graphEdges, ['dfs(A)', 'dfs(B)', 'dfs(C)']),
      nodeFrame('无新邻居则回溯', 'C 的 E 尚未访问，访问 E；之后回到 B。', 'if !visited[next] dfs(next)', graphNodes({ A: 'done', B: 'active', C: 'done', E: 'done' }), graphEdges, ['dfs(A)', 'dfs(B)']),
      nodeFrame('覆盖整个连通分量', '最终从已有栈分支访问 D，每个结点只首次展开。', '总复杂度 O(V+E)', graphNodes({ A: 'done', B: 'done', C: 'done', D: 'done', E: 'done' }), graphEdges, ['A → B → C → E → D'], '记忆点：入递归前立即标记，保证每个结点只展开一次。')
    ]
  },
  {
    id: 'graph-bfs', title: '图的广度优先搜索', kind: 'graph', frames: [
      nodeFrame('起点入队并标记', 'A 的距离为 0；入队时就标记，避免重复排队。', 'queue.push(A); dist[A]=0', graphNodes({ A: 'active' }), graphEdges, ['队列: A']),
      nodeFrame('访问一跳邻居', '弹出 A，将 B、D 设为距离 1。', 'dist[next] = dist[node] + 1', graphNodes({ A: 'done', B: 'candidate', D: 'candidate' }), graphEdges, ['队列: B, D']),
      nodeFrame('按距离顺序扩张', '先处理 B，发现 C；E 可能随后由 D 发现。', 'node = queue.pop_front()', graphNodes({ A: 'done', B: 'done', D: 'active', C: 'candidate', E: 'candidate' }), graphEdges, ['队列: D, C, E']),
      nodeFrame('得到无权最短路', '首次到达的层数就是最短边数。', '首次访问即确定最短距离', graphNodes({ A: 'done', B: 'done', C: 'done', D: 'done', E: 'done' }), graphEdges, ['距离: 0 | 1,1 | 2,2'], '记忆点：BFS 队列按距离分层，首次到达即最短。')
    ]
  },
  {
    id: 'topological-sort', title: '拓扑排序', kind: 'graph', frames: [
      nodeFrame('统计每个结点入度', '边表示“先修 → 后续”，只有入度 0 的任务可开始。', 'indegree[v]++', graphNodes({ A: 'candidate' }), graphEdges, ['入度0: A']),
      nodeFrame('取出 A', '输出 A，并删除它的出边，B、D 的入度各减 1。', 'for next: indegree[next]--', graphNodes({ A: 'done', B: 'candidate', D: 'candidate' }), graphEdges, ['结果: A', '队列: B, D']),
      nodeFrame('新的入度 0 入队', '处理 B 后 C 可用；处理 D 后 E 仍等待 C。', 'if indegree[next] == 0 push', graphNodes({ A: 'done', B: 'done', D: 'done', C: 'candidate', E: 'normal' }), graphEdges, ['结果: A,B,D', '队列: C']),
      nodeFrame('完成并检查环', '输出数等于结点数，得到合法顺序；否则图中存在环。', 'order.length == V', graphNodes({ A: 'done', B: 'done', C: 'done', D: 'done', E: 'done' }), graphEdges, ['A, B, D, C, E'], '记忆点：拓扑排序不断删除入度 0 的结点；删不动却没删完就是有环。')
    ]
  },
  arraySpec('union-find', '并查集路径压缩', 'graph', [
    { title: '每个点自成集合', description: 'parent[i]=i，根结点代表集合。', values: [0, 1, 2, 3, 4, 5], done: [0, 1, 2, 3, 4, 5], code: 'parent[i] = i' },
    { title: '合并两个集合', description: 'union(1,2) 把 2 的根接到 1 的根。', values: [0, 1, 1, 3, 4, 5], active: [1, 2], code: 'parent[rootB] = rootA' },
    { title: '形成较长父链', description: '继续合并后，5 的路径可能是 5→4→3→1。', values: [0, 1, 1, 1, 3, 4], candidate: [3, 4, 5], pointers: [{ label: 'find(5)', index: 5 }], code: 'find(x) follows parent[x]' },
    { title: '路径压缩', description: '一次 find(5) 后，沿途 5、4 都直接指向根 1。', values: [0, 1, 1, 1, 1, 1], done: [1, 3, 4, 5], code: 'parent[x] = find(parent[x])', takeaway: '记忆点：find 不只查根，还顺手把回程路径压平。' }
  ]),
  {
    id: 'dijkstra', title: 'Dijkstra 最短路', kind: 'graph', frames: [
      nodeFrame('起点距离为 0', '其余距离先设为无穷，最小堆放入 (0,A)。', 'dist[A]=0; heap.push(A)', graphNodes({ A: 'candidate' }), graphEdges, ['堆: (0,A)']),
      nodeFrame('取出当前最近结点', 'A 距离最小并定型，松弛相邻边得到 B=2、D=5。', 'new = dist[A] + weight', graphNodes({ A: 'done', B: 'candidate', D: 'candidate' }), graphEdges, ['dist: A0 B2 D5']),
      nodeFrame('用更短路径更新', '取 B 后，经 B 到 D 的距离 2+1=3，比原 5 更短。', 'if new < dist[next] update', graphNodes({ A: 'done', B: 'done', D: 'active', C: 'candidate' }), graphEdges, ['堆: (3,D), (5,D), (6,C)']),
      nodeFrame('忽略过期堆项', '弹出旧的 (5,D) 时发现大于 dist[D]=3，直接跳过。', 'if popped > dist[node] continue', graphNodes({ A: 'done', B: 'done', C: 'done', D: 'done', E: 'done' }), graphEdges, ['最终距离: 0,2,6,3,5'], '记忆点：不必修改堆中旧值；弹出时判断是否过期即可。')
    ]
  },
  {
    id: 'kruskal', title: 'Kruskal 最小生成树', kind: 'graph', frames: [
      nodeFrame('边按权重排序', '从最便宜的边开始考虑，初始每个点各自连通。', 'sort(edges by weight)', graphNodes(), graphEdges.map((edge, i) => ({ ...edge, label: String([1, 2, 3, 4, 5, 6][i]) })), ['权重顺序: 1,2,3,4,5,6']),
      nodeFrame('加入不成环的边', 'A-B 权重 1，两个端点不连通，因此加入。', 'if find(u) != find(v)', graphNodes({ A: 'done', B: 'done' }), graphEdges.map((edge, i) => ({ ...edge, state: i === 0 ? 'done' : 'normal' })), ['MST: A-B']),
      nodeFrame('跳过会成环的边', '若 A、B、D 已连通，再选 B-D 会形成环，跳过。', 'else skip edge', graphNodes({ A: 'done', B: 'done', D: 'done' }), graphEdges.map((edge, i) => ({ ...edge, state: i < 2 ? 'done' : i === 3 ? 'active' : 'normal' })), ['并查集判断连通']),
      nodeFrame('选满 V-1 条边', '所有结点连通后立即结束，得到总权重最小的树。', 'if chosen == V-1 break', graphNodes({ A: 'done', B: 'done', C: 'done', D: 'done', E: 'done' }), graphEdges.map((edge, i) => ({ ...edge, state: [0, 1, 2, 4].includes(i) ? 'done' : 'muted' })), ['已选 4 条边'], '记忆点：排序保证当前最便宜，并查集保证永远不成环。')
    ]
  },
  {
    id: 'knapsack-01', title: '0/1 背包状态转移', kind: 'table', frames: [
      { title: '定义一维状态', description: 'dp[c] 表示容量不超过 c 时的最大价值。', codeLine: 'dp[c] = max value within capacity c', table: { columns: ['0', '1', '2', '3', '4', '5'], rows: [{ label: 'dp', values: ['0', '0', '0', '0', '0', '0'] }], active: [] } },
      { title: '处理物品 (重2,值3)', description: '容量 2..5 可以由“原容量减 2”转移。', codeLine: 'dp[c] = max(dp[c], dp[c-2]+3)', table: { columns: ['0', '1', '2', '3', '4', '5'], rows: [{ label: 'dp', values: ['0', '0', '3', '3', '3', '3'] }], active: [[0, 2], [0, 3], [0, 4], [0, 5]] } },
      { title: '容量必须倒序', description: '处理物品 (重3,值4) 时从 5 向 3 更新，避免本轮重复使用。', codeLine: 'for c = capacity downTo weight', table: { columns: ['0', '1', '2', '3', '4', '5'], rows: [{ label: 'dp', values: ['0', '0', '3', '4', '4', '7'] }], active: [[0, 5], [0, 4], [0, 3]] } },
      { title: '容量 5 得到最优 7', description: '它来自容量 2 的价值 3，再放入重量 3 的物品。', codeLine: 'dp[5] = dp[2] + 4 = 7', table: { columns: ['0', '1', '2', '3', '4', '5'], rows: [{ label: 'dp', values: ['0', '0', '3', '4', '4', '7'] }], active: [[0, 2], [0, 5]] }, takeaway: '记忆点：0/1 背包倒序是为了让转移读取“上一轮物品”的状态。' }
    ]
  },
  arraySpec('kmp', 'KMP 前缀回退', 'string', [
    { title: '模式串与文本对齐', description: '在文本 ABABAC 中查找模式 ABAC。', values: ['A', 'B', 'A', 'B', 'A', 'C'], active: [0, 1, 2], pointers: [{ label: 'i', index: 2 }, { label: 'j', index: 2 }], code: 'if text[i] == pattern[j] { i++; j++ }' },
    { title: '出现失配', description: '文本第 4 个字符 B 与模式第 4 个字符 C 不同。', values: ['A', 'B', 'A', 'B', 'A', 'C'], pivot: [3], pointers: [{ label: 'i', index: 3 }, { label: 'j', index: 3 }], code: 'mismatch at j=3' },
    { title: 'j 按前缀表回退', description: '已匹配 ABA 的最长相等前后缀是 A，因此 j 回到 1，i 不回退。', values: ['A', 'B', 'A', 'B', 'A', 'C'], candidate: [3], pointers: [{ label: 'i不动', index: 3 }, { label: 'j=1', index: 1 }], code: 'j = lps[j - 1]' },
    { title: '复用已有匹配继续', description: '从文本 B 与模式 B 继续，最终匹配 ABAC。', values: ['A', 'B', 'A', 'B', 'A', 'C'], done: [2, 3, 4, 5], code: 'j == pattern.length → found', takeaway: '记忆点：失配时文本指针不回头，模式指针跳到可复用前缀长度。' }
  ]),
  {
    id: 'segment-tree-lazy', title: '线段树懒标记', kind: 'tree', frames: [
      nodeFrame('根维护整个区间', '每个结点保存区间和；孩子把区间一分为二。', 'tree[node] = sum(l..r)', [
        { id: '14', label: '[1,4] 10', x: 50, y: 12, state: 'active' }, { id: '12', label: '[1,2] 3', x: 28, y: 46, state: 'normal' }, { id: '34', label: '[3,4] 7', x: 72, y: 46, state: 'normal' }, { id: '1', label: '1', x: 15, y: 82, state: 'normal' }, { id: '2', label: '2', x: 39, y: 82, state: 'normal' }, { id: '3', label: '3', x: 62, y: 82, state: 'normal' }, { id: '4', label: '4', x: 85, y: 82, state: 'normal' }
      ], [{ from: '14', to: '12', state: 'normal' }, { from: '14', to: '34', state: 'normal' }, { from: '12', to: '1', state: 'normal' }, { from: '12', to: '2', state: 'normal' }, { from: '34', to: '3', state: 'normal' }, { from: '34', to: '4', state: 'normal' }], ['array = [1,2,3,4]']),
      nodeFrame('整段覆盖直接更新', '区间 [1,2] 整体 +3，结点和增加 3×2，并记录 lazy=3。', 'tree[node] += delta * length', [
        { id: '14', label: '[1,4] 16', x: 50, y: 12, state: 'candidate' }, { id: '12', label: '[1,2] 9 lazy+3', x: 28, y: 46, state: 'active' }, { id: '34', label: '[3,4] 7', x: 72, y: 46, state: 'normal' }, { id: '1', label: '1', x: 15, y: 82, state: 'muted' }, { id: '2', label: '2', x: 39, y: 82, state: 'muted' }, { id: '3', label: '3', x: 62, y: 82, state: 'normal' }, { id: '4', label: '4', x: 85, y: 82, state: 'normal' }
      ], treeEdgesForSegment(), ['lazy[1,2] = 3']),
      nodeFrame('需要下探时再下放', '查询单点 2 前，把 lazy 传给两个孩子并清空父标记。', 'pushDown(node)', [
        { id: '14', label: '[1,4] 16', x: 50, y: 12, state: 'normal' }, { id: '12', label: '[1,2] 9', x: 28, y: 46, state: 'done' }, { id: '34', label: '[3,4] 7', x: 72, y: 46, state: 'normal' }, { id: '1', label: '4', x: 15, y: 82, state: 'candidate' }, { id: '2', label: '5', x: 39, y: 82, state: 'active' }, { id: '3', label: '3', x: 62, y: 82, state: 'normal' }, { id: '4', label: '4', x: 85, y: 82, state: 'normal' }
      ], treeEdgesForSegment(), ['lazy children += 3']),
      nodeFrame('只访问相关分支', '查询 [2,4] 复用完整覆盖结点，结果为 5+3+4=12。', 'query returns covered node directly', [
        { id: '14', label: '[1,4] 16', x: 50, y: 12, state: 'normal' }, { id: '12', label: '[1,2] 9', x: 28, y: 46, state: 'candidate' }, { id: '34', label: '[3,4] 7', x: 72, y: 46, state: 'done' }, { id: '1', label: '4', x: 15, y: 82, state: 'muted' }, { id: '2', label: '5', x: 39, y: 82, state: 'done' }, { id: '3', label: '3', x: 62, y: 82, state: 'done' }, { id: '4', label: '4', x: 85, y: 82, state: 'done' }
      ], treeEdgesForSegment(), ['query(2,4)=12'], '记忆点：懒标记不是不更新，而是把“对子树的统一操作”延迟到必须下探时。')
    ]
  }
];

function treeEdgesForSegment(): VisualEdge[] {
  return [
    ['14', '12'], ['14', '34'], ['12', '1'], ['12', '2'], ['34', '3'], ['34', '4']
  ].map(([from, to]) => ({ from, to, state: 'normal' }));
}

export const visualizationById = new Map(visualizationSpecs.map((spec) => [spec.id, spec]));

export function getVisualization(id: string): VisualizationSpec | undefined {
  return visualizationById.get(id);
}

