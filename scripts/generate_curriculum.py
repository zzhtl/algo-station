#!/usr/bin/env python3
"""Build the deterministic curated curriculum catalog from bundled articles and local problems."""

from __future__ import annotations

import html
import json
import re
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ARTICLES_DIR = ROOT / "content" / "articles"
LEETCODE_DIR = ROOT / "data" / "leetcode" / "solution"
OUTPUT = ROOT / "content" / "curriculum" / "catalog.json"


@dataclass(frozen=True)
class StageSpec:
    id: str
    title: str
    description: str
    lesson_count: int
    exercise_count: int
    categories: tuple[str, ...]
    keywords: tuple[str, ...] = ()


STAGES = (
    StageSpec("stage-01", "复杂度、递归与排序", "先建立复杂度、边界和排序过程的可解释模型。", 8, 12,
              ("基础与排序",),
              ("排序", "划分", "归并", "快速", "扫描", "枚举", "桶", "哨兵")),
    StageSpec("stage-02", "数组、字符串与双指针", "掌握线性结构上的指针、窗口、分组和矩阵模拟。", 10, 15,
              ("数组与字符串",), ("指针", "窗口", "数组", "字符串", "矩阵")),
    StageSpec("stage-03", "哈希、前缀和与区间扫描", "用索引、计数和前缀信息消除重复计算。", 8, 12,
              ("哈希与前缀", "区间查询", "哈希表", "数组与字符串"), ("哈希", "前缀", "差分", "区间")),
    StageSpec("stage-04", "链表、栈、队列与堆", "训练指针重连、受限访问顺序和优先级维护。", 10, 15,
              ("链表", "栈与队列", "堆与优先队列"), ("反转", "环", "单调", "队列", "堆", "合并")),
    StageSpec("stage-05", "二分、分治与选择", "把单调性和候选空间缩减写成可证明的循环。", 7, 10,
              ("二分搜索", "数学与位运算"), ("二分", "边界", "划分", "选择")),
    StageSpec("stage-06", "二叉树、BST 与 Trie", "围绕递归返回值、遍历顺序和树上状态组织解法。", 10, 15,
              ("二叉树", "设计与数据结构", "字典树"), ("遍历", "BST", "树", "Trie", "字典")),
    StageSpec("stage-07", "回溯、剪枝与约束搜索", "用选择、路径、撤销和约束传播控制搜索树。", 8, 12,
              ("回溯",), ("组合", "排列", "子集", "剪枝", "约束")),
    StageSpec("stage-08", "图搜索、连通性与最短路", "从建图开始掌握遍历、拓扑、连通性和路径算法。", 10, 15,
              ("搜索与图论", "并查集"), ("DFS", "BFS", "拓扑", "最短路", "并查集", "生成树")),
    StageSpec("stage-09", "贪心、区间与调度", "通过交换论证和局部选择解决区间与资源分配。", 7, 10,
              ("贪心",), ("区间", "调度", "交换", "优先")),
    StageSpec("stage-10", "动态规划核心", "从状态含义、转移来源和遍历顺序推导动态规划。", 10, 16,
              ("动态规划",), ("状态", "背包", "子序列", "区间", "滚动")),
    StageSpec("stage-11", "字符串与区间高级结构", "掌握字符串复用信息和动态区间维护结构。", 6, 9,
              ("字符串匹配", "区间查询", "设计与数据结构", "哈希与前缀"), ("KMP", "Z", "Manacher", "树状", "线段树", "哈希")),
    StageSpec("stage-12", "高级 DP、数学与综合", "综合位运算、计数、状态压缩和高维动态规划。", 6, 9,
              ("进阶动态规划", "数学与位运算", "动态规划"), ("数位", "概率", "状态压缩", "重根", "快速幂")),
)


# 固定课程顺序是教学设计的一部分，不能由文章数量或标题排序的变化隐式改写。
PINNED_LESSONS = {
    "stage-01": (
        "foundation-01-complexity-invariant", "foundation-02-recursion-stack",
        "foundation-03-insertion-sort", "foundation-04-merge-sort",
        "foundation-05-quick-partition", "foundation-06-linear-sorts",
        "foundation-07-stability-comparator", "foundation-08-choose-sorting",
    ),
    "stage-02": (
        "array-string-01-two-end-pointers", "array-string-02-same-direction-overwrite",
        "array-string-03-sliding-window-count", "array-string-06-inplace-swap-stable-write",
        "array-string-07-prefix-suffix-product", "array-string-08-difference-array",
        "array-string-09-circular-array-mod", "array-string-10-matrix-layer-simulation",
        "array-string-13-sentinel-boundary", "array-string-14-interval-events",
    ),
    "stage-03": (
        "hash-map-patterns", "prefix-sum-diff", "hash-prefix-201-first-seen-index",
        "hash-prefix-202-frequency-bucket", "hash-prefix-204-difference-of-counts",
        "hash-prefix-206-pair-complement-count", "hash-prefix-207-prefix-sum-2d",
        "hash-prefix-208-canonical-key",
    ),
    "stage-04": (
        "linked-list-reverse", "linked-list-05-topic-669764", "linked-list-10-topic-842522",
        "monotonic-stack", "stack-queue-05-topic-1259267",
        "stack-queue-204-deque-sliding-window", "heap-priority-queue",
        "heap-priority-01-top-k", "heap-priority-03-topic-1345868", "heap-priority-05-topic-1293249",
    ),
    "stage-05": (
        "binary-search-01-topic-1495886", "binary-search-02-topic-1506640",
        "binary-search-03-topic-1116927", "binary-search-04-topic-1409855",
        "binary-search-05-topic-1002843", "binary-search-06-topic-1428801",
        "binary-search-07-topic-1041406",
    ),
    "stage-06": (
        "binary-tree-01-topic-717705", "binary-tree-02-topic-1027010",
        "binary-tree-03-topic-1260447", "binary-tree-04-topic-1027132",
        "binary-tree-05-topic-837058", "binary-tree-06-topic-1338560",
        "binary-tree-07-topic-866225", "binary-tree-08-topic-802390",
        "binary-tree-09-topic-1014882", "design-structure-02-trie",
    ),
    "stage-07": (
        "backtracking-01-used", "backtracking-02-start",
        "backtracking-03-topic-1466158", "backtracking-04-topic-1322033",
        "backtracking-08-n", "backtracking-09-topic-1518506",
        "backtracking-202-combination-start-index", "backtracking-203-subset-include-exclude",
    ),
    "stage-08": (
        "graph-search-01-dfs", "graph-search-02-bfs", "graph-search-03-topic-758194",
        "union-find", "dijkstra-shortest-path", "graph-search-208-minimum-spanning-tree",
        "graph-search-04-bfs", "graph-search-07-bfs", "graph-search-11-floyd",
        "graph-search-205-bipartite-coloring",
    ),
    "stage-09": (
        "greedy-intro", "greedy-207-local-swap-invariant",
        "greedy-201-interval-end-first", "greedy-202-priority-queue-deadline",
        "greedy-02-topic-1718610", "greedy-03-topic-1182937", "greedy-04-topic-950574",
    ),
    "stage-10": (
        "dp-intro", "dynamic-programming-201-linear-state-machine", "knapsack-dp",
        "dynamic-programming-03-topic-935709", "dynamic-programming-04-topic-924489",
        "dynamic-programming-01-topic-1325327", "dynamic-programming-05-dp",
        "dynamic-programming-07-dp", "dynamic-programming-09-topic-1603536",
        "dynamic-programming-13-topic-935832",
    ),
    "stage-11": (
        "string-matching-01-kmp", "string-matching-03-z", "string-matching-04-manacher",
        "hash-prefix-203-rolling-hash-window", "range-query-01-topic-1779358",
        "design-structure-203-segment-tree-lazy",
    ),
    "stage-12": (
        "advanced-dp-01-dp", "advanced-dp-03-dp", "advanced-dp-04-dp",
        "advanced-dp-07-dp", "advanced-dp-08-dp", "advanced-dp-10-topic-1359339",
    ),
}


VISUALIZATIONS = {
    "stage-01": (("insertion-sort", "插入排序", "array"), ("merge-sort", "归并排序", "array"),
                 ("quick-sort-partition", "快速排序划分", "array")),
    "stage-02": (("two-pointers", "相向双指针", "array"), ("sliding-window", "滑动窗口", "array")),
    "stage-03": (("prefix-difference", "前缀和与差分", "array"),),
    "stage-04": (("linked-list-reverse", "链表反转", "linked_list"),
                 ("fast-slow-cycle", "快慢指针判环", "linked_list"),
                 ("monotonic-stack", "单调栈", "stack"), ("binary-heap", "堆的插入与删除", "heap")),
    "stage-05": (("binary-search-boundary", "二分边界", "array"),
                 ("binary-search-answer", "答案二分", "array")),
    "stage-06": (("tree-dfs", "二叉树深度优先遍历", "tree"), ("tree-bfs", "二叉树层序遍历", "tree"),
                 ("trie", "Trie 插入与查找", "trie")),
    "stage-07": (("backtracking-tree", "回溯决策树", "tree"),),
    "stage-08": (("graph-dfs", "图的深度优先搜索", "graph"), ("graph-bfs", "图的广度优先搜索", "graph"),
                 ("topological-sort", "拓扑排序", "graph"), ("union-find", "并查集路径压缩", "graph"),
                 ("dijkstra", "Dijkstra 最短路", "graph"), ("kruskal", "Kruskal 最小生成树", "graph")),
    "stage-10": (("knapsack-01", "0/1 背包状态转移", "table"),),
    "stage-11": (("kmp", "KMP 前缀回退", "string"), ("segment-tree-lazy", "线段树懒标记", "tree")),
}


VISUALIZATION_ARTICLES = {
    "insertion-sort": "foundation-03-insertion-sort",
    "merge-sort": "foundation-04-merge-sort",
    "quick-sort-partition": "foundation-05-quick-partition",
    "two-pointers": "array-string-01-two-end-pointers",
    "sliding-window": "array-string-03-sliding-window-count",
    "prefix-difference": "prefix-sum-diff",
    "linked-list-reverse": "linked-list-reverse",
    "fast-slow-cycle": "linked-list-05-topic-669764",
    "monotonic-stack": "monotonic-stack",
    "binary-heap": "heap-priority-queue",
    "binary-search-boundary": "binary-search-01-topic-1495886",
    "binary-search-answer": "binary-search-03-topic-1116927",
    "tree-dfs": "binary-tree-01-topic-717705",
    "tree-bfs": "binary-tree-02-topic-1027010",
    "trie": "design-structure-02-trie",
    "backtracking-tree": "backtracking-01-used",
    "graph-dfs": "graph-search-01-dfs",
    "graph-bfs": "graph-search-02-bfs",
    "topological-sort": "graph-search-03-topic-758194",
    "union-find": "union-find",
    "dijkstra": "dijkstra-shortest-path",
    "kruskal": "graph-search-208-minimum-spanning-tree",
    "knapsack-01": "knapsack-dp",
    "kmp": "string-matching-01-kmp",
    "segment-tree-lazy": "design-structure-203-segment-tree-lazy",
}


def parse_frontmatter(path: Path) -> dict:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise ValueError(f"missing frontmatter: {path}")
    raw, body = text[4:].split("\n---\n", 1)
    result: dict[str, object] = {"slug": path.stem, "body_length": len(body)}
    for line in raw.splitlines():
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        value = value.strip().strip("\"'")
        if key == "problem_ids":
            result[key] = [int(item.strip()) for item in value.strip("[]").split(",") if item.strip()]
        elif key == "order":
            result[key] = int(value)
        else:
            result[key] = value
    return result


def article_rank(article: dict, stage: StageSpec) -> tuple:
    title = str(article["title"])
    keyword_rank = 0 if any(word.lower() in title.lower() for word in stage.keywords) else 1
    order = int(article.get("order", 9999))
    quality_band = 0 if 101 <= order <= 114 else 1 if order <= 20 else 2
    return keyword_rank, quality_band, order, -int(article["body_length"]), str(article["slug"])


def select_articles(all_articles: list[dict]) -> list[tuple[StageSpec, list[dict]]]:
    by_slug = {str(article["slug"]): article for article in all_articles}
    used: set[str] = set()
    selected: list[tuple[StageSpec, list[dict]]] = []
    for stage in STAGES:
        pinned = PINNED_LESSONS.get(stage.id, ())
        missing = [slug for slug in pinned if slug not in by_slug]
        if missing:
            raise RuntimeError(f"{stage.id}: missing pinned articles: {missing}")
        items = [by_slug[slug] for slug in pinned]
        if len(items) != stage.lesson_count:
            raise RuntimeError(f"{stage.id}: expected {stage.lesson_count} articles, got {len(items)}")
        duplicate = next((str(item["slug"]) for item in items if item["slug"] in used), None)
        if duplicate:
            raise RuntimeError(f"{stage.id}: duplicate pinned article: {duplicate}")
        invalid = [str(item["slug"]) for item in items if not item.get("problem_ids") or int(item["body_length"]) < 700]
        if invalid:
            raise RuntimeError(f"{stage.id}: pinned articles fail content contract: {invalid}")
        used.update(str(item["slug"]) for item in items)
        selected.append((stage, items))
    return selected


def problem_readme(problem_id: int) -> Path | None:
    prefix = f"{problem_id:04d}."
    bucket = LEETCODE_DIR / f"{problem_id // 100 * 100:04d}-{problem_id // 100 * 100 + 99:04d}"
    if not bucket.is_dir():
        return None
    for directory in sorted(bucket.iterdir()):
        if directory.name.startswith(prefix):
            candidate = directory / "README_EN.md"
            if candidate.is_file():
                return candidate
    return None


def strip_markup(value: str) -> str:
    value = re.sub(r"<br\s*/?>", "\n", value, flags=re.I)
    value = re.sub(r"<[^>]+>", "", value)
    return html.unescape(value).replace("\xa0", " ").strip()


def problem_metadata(problem_id: int) -> dict | None:
    readme = problem_readme(problem_id)
    if readme is None:
        return None
    text = readme.read_text(encoding="utf-8")
    title_match = re.search(rf"^# \[{problem_id}\. ([^]]+)\]", text, re.M)
    difficulty_match = re.search(r"^difficulty:\s*(\w+)", text, re.M)
    examples: list[tuple[str, str]] = []
    blocks = re.findall(r"<pre>(.*?)</pre>", text, flags=re.S | re.I)
    blocks.extend(re.findall(r'<div class="example-block">(.*?)</div>', text, flags=re.S | re.I))
    for block in blocks:
        plain = strip_markup(block)
        match = re.search(
            r"Input:?\s*(.*?)\nOutput:?\s*(.*?)(?:\nExplanation:?|\Z)",
            plain,
            flags=re.S,
        )
        if match:
            input_value = match.group(1).strip()
            output_value = match.group(2).strip()
            example = (input_value, output_value)
            if input_value and output_value and example not in examples:
                examples.append(example)
    if not examples:
        return None
    return {
        "title": title_match.group(1).strip() if title_match else f"Problem {problem_id}",
        "difficulty": difficulty_match.group(1) if difficulty_match else "Medium",
        "examples": examples,
    }


def starter_templates(title: str) -> list[dict]:
    return [
        {
            "language": "go",
            "contract": "function",
            "code": f'''package main\n\n// Solve receives the problem input and returns the required output.\n// Exercise: {title}\nfunc Solve(input string) string {{\n\treturn ""\n}}\n''',
        },
        {
            "language": "go",
            "contract": "stdio",
            "code": f'''package main\n\nimport (\n\t"bufio"\n\t"fmt"\n\t"os"\n)\n\n// Exercise: {title}\nfunc main() {{\n\tin := bufio.NewReader(os.Stdin)\n\t_ = in\n\tfmt.Print("")\n}}\n''',
        },
        {
            "language": "rust",
            "contract": "function",
            "code": f'''/// Receives the problem input and returns the required output.\n/// Exercise: {title}\npub fn solve(input: &str) -> String {{\n    let _ = input;\n    String::new()\n}}\n''',
        },
        {
            "language": "rust",
            "contract": "stdio",
            "code": f'''use std::io::{{self, Read}};\n\n// Exercise: {title}\nfn main() {{\n    let mut input = String::new();\n    io::stdin().read_to_string(&mut input).expect("read stdin");\n    let _ = input;\n    print!("");\n}}\n''',
        },
    ]


def quiz_for(lesson_slug: str, title: str, summary: str) -> list[dict]:
    return [
        {
            "id": f"{lesson_slug}-q1",
            "prompt": f"学习“{title}”时，写代码前最应该先明确什么？",
            "options": ["核心状态或不变量", "变量名长度", "使用哪种编辑器", "代码行数"],
            "correct_index": 0,
            "explanation": "先明确状态、不变量和移动理由，模板才不会变成机械记忆。",
        },
        {
            "id": f"{lesson_slug}-q2",
            "prompt": "分析算法复杂度时，哪项做法更可靠？",
            "options": ["只看循环层数", "按每个元素或状态被处理的次数推导", "记住题解结论", "忽略额外空间"],
            "correct_index": 1,
            "explanation": "摊还、剪枝和数据结构操作不能只靠循环外观判断。",
        },
        {
            "id": f"{lesson_slug}-q3",
            "prompt": f"下面哪种复盘最能帮助迁移本节方法？（提示：{summary[:36]}）",
            "options": ["背完整代码", "只记录是否通过", "写出识别信号、边界和失效条件", "收藏题目后不再回看"],
            "correct_index": 2,
            "explanation": "识别信号和失效条件比记住某一道题的实现更容易迁移。",
        },
    ]


def build_catalog() -> dict:
    articles = [parse_frontmatter(path) for path in sorted(ARTICLES_DIR.glob("*.md"))]
    selection = select_articles(articles)
    stages: list[dict] = []
    lessons: list[dict] = []
    exercises: list[dict] = []
    visualizations: list[dict] = []
    used_problem_ids: set[int] = set()
    previous_lesson: str | None = None

    for stage_index, (stage, stage_articles) in enumerate(selection, start=1):
        stage_lesson_slugs: list[str] = []
        stage_lessons: list[dict] = []
        visualization_specs = VISUALIZATIONS.get(stage.id, ())
        stage_article_lessons: dict[str, str] = {}

        for lesson_index, article in enumerate(stage_articles, start=1):
            slug = f"{stage.id}-lesson-{lesson_index:02d}"
            stage_lesson_slugs.append(slug)
            stage_article_lessons[str(article["slug"])] = slug
            visualization_id = next(
                (item[0] for item in visualization_specs if VISUALIZATION_ARTICLES[item[0]] == article["slug"]),
                None,
            )
            lesson = {
                "slug": slug,
                "stage_id": stage.id,
                "article_slug": article["slug"],
                "title": article["title"],
                "summary": article.get("summary", ""),
                "order": lesson_index,
                "estimated_minutes": 25,
                "prerequisites": [previous_lesson] if previous_lesson else [],
                "visualization_id": visualization_id,
                "objectives": [
                    f"用自己的话解释“{article['title']}”的核心状态或不变量",
                    "能从约束推导时间与空间复杂度",
                    "能识别至少一个易错边界和一个迁移场景",
                ],
                "quiz": quiz_for(slug, str(article["title"]), str(article.get("summary", ""))),
                "exercise_slugs": [],
                "core_exercise_slugs": [],
            }
            stage_lessons.append(lesson)
            lessons.append(lesson)
            previous_lesson = slug

        candidate_pairs: list[tuple[dict, int, dict]] = []
        stage_problem_ids: set[int] = set()
        for lesson, article in zip(stage_lessons, stage_articles, strict=True):
            for problem_id in article.get("problem_ids", []):
                if problem_id in used_problem_ids or problem_id in stage_problem_ids:
                    continue
                metadata = problem_metadata(problem_id)
                if metadata is not None and len(metadata["examples"]) >= 2:
                    candidate_pairs.append((lesson, problem_id, metadata))
                    stage_problem_ids.add(problem_id)

        selected_article_slugs = {str(article["slug"]) for article in stage_articles}
        supplemental_articles = [
            article for article in articles
            if article.get("category") in stage.categories
            and article["slug"] not in selected_article_slugs
            and article.get("problem_ids")
        ]
        supplemental_articles.sort(key=lambda article: article_rank(article, stage))
        for article in supplemental_articles:
            for problem_id in article.get("problem_ids", []):
                if problem_id in used_problem_ids or problem_id in stage_problem_ids:
                    continue
                metadata = problem_metadata(problem_id)
                if metadata is not None and len(metadata["examples"]) >= 2:
                    lesson = stage_lessons[len(stage_problem_ids) % len(stage_lessons)]
                    candidate_pairs.append((lesson, problem_id, metadata))
                    stage_problem_ids.add(problem_id)

        # Ensure every lesson receives one core exercise before distributing extensions.
        allocated: list[tuple[dict, int, dict]] = []
        remaining = candidate_pairs.copy()
        for lesson in stage_lessons:
            index = next((i for i, item in enumerate(remaining) if item[0] is lesson), None)
            if index is None:
                if not remaining:
                    raise RuntimeError(f"{lesson['slug']} has no parseable unique problem example")
                _, problem_id, metadata = remaining.pop(0)
                allocated.append((lesson, problem_id, metadata))
            else:
                allocated.append(remaining.pop(index))
        allocated.extend(remaining[: stage.exercise_count - len(allocated)])
        if len(allocated) != stage.exercise_count:
            raise RuntimeError(f"{stage.id}: expected {stage.exercise_count} exercises, got {len(allocated)}")

        for lesson, problem_id, metadata in allocated:
            exercise_slug = f"exercise-{problem_id:04d}"
            examples = metadata["examples"]
            visible_input, visible_output = examples[0]
            hidden_input, hidden_output = examples[1]
            exercise = {
                "slug": exercise_slug,
                "problem_id": problem_id,
                "lesson_slug": lesson["slug"],
                "title": metadata["title"],
                "difficulty": metadata["difficulty"],
                "summary": f"围绕“{lesson['title']}”完成 {metadata['title']}，分别验证函数与标准输入输出写法。",
                "starters": starter_templates(metadata["title"]),
                "cases": [
                    {"name": "公开样例", "visibility": "public", "input": visible_input, "expected": visible_output},
                    {"name": "隐藏回归", "visibility": "hidden", "input": hidden_input, "expected": hidden_output},
                ],
                "limits": {"compile_ms": 20000, "case_ms": 2000, "total_ms": 30000, "memory_mb": 256, "output_kb": 64},
            }
            exercises.append(exercise)
            lesson["exercise_slugs"].append(exercise_slug)
            if not lesson["core_exercise_slugs"]:
                lesson["core_exercise_slugs"].append(exercise_slug)
            used_problem_ids.add(problem_id)

        stages.append({
            "id": stage.id,
            "title": stage.title,
            "description": stage.description,
            "order": stage_index,
            "lesson_slugs": stage_lesson_slugs,
        })
        for visualization_id, title, kind in visualization_specs:
            article_slug = VISUALIZATION_ARTICLES[visualization_id]
            lesson_slug = stage_article_lessons.get(article_slug)
            if lesson_slug is None:
                raise RuntimeError(f"{stage.id}: visualization {visualization_id} has no matching lesson")
            visualizations.append({
                "id": visualization_id,
                "lesson_slug": lesson_slug,
                "title": title,
                "kind": kind,
                "description": f"用预设案例逐步演示{title}，同步显示状态、变量和伪代码位置。",
            })

    return {
        "schema_version": 1,
        "stages": stages,
        "lessons": lessons,
        "exercises": exercises,
        "visualizations": visualizations,
    }


def main() -> None:
    catalog = build_catalog()
    expected = (12, 100, 150, 25)
    actual = tuple(len(catalog[key]) for key in ("stages", "lessons", "exercises", "visualizations"))
    if actual != expected:
        raise RuntimeError(f"catalog counts {actual}, expected {expected}")
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(catalog, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"generated {OUTPUT.relative_to(ROOT)}: stages/lessons/exercises/visualizations={actual}")


if __name__ == "__main__":
    main()
