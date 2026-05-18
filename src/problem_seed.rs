//! 从瘦身后的 `data/leetcode/`（doocs/leetcode 仅保留 README.md / README_EN.md）扫盘，
//! 解析 YAML front matter + H1，建立 SQLite 索引（problems / tags / problem_tags / problems_fts）。
//!
//! 不联网。题面 markdown 内容不入库，运行时由 routes::problems::statement 直接读文件。

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use sqlx::SqlitePool;

#[derive(Debug, Default)]
pub struct SeedStats {
    pub problems: usize,
    pub tags: usize,
    pub skipped_no_readme: usize,
    pub skipped_parse_failed: usize,
}

/// 默认 leetcode 仓库路径解析顺序：env(LEETCODE_REPO_DIR) → exe_dir/data/leetcode → ./data/leetcode。
pub fn resolve_repo_path() -> Option<PathBuf> {
    if let Ok(env) = std::env::var("LEETCODE_REPO_DIR") {
        let p = PathBuf::from(env);
        if p.join("solution").is_dir() {
            return Some(p);
        }
    }
    let near_exe = crate::exe_dir().join("data").join("leetcode");
    if near_exe.join("solution").is_dir() {
        return Some(near_exe);
    }
    let dev = PathBuf::from("data/leetcode");
    if dev.join("solution").is_dir() {
        return Some(dev);
    }
    None
}

/// 全量扫描 `repo/solution/<bucket>/<题目目录>/`，重建 SQLite 索引。
/// 会先 DELETE 所有相关表再写入（保持 idempotent）。
pub async fn seed_from_repo(pool: &SqlitePool, repo: &Path) -> Result<SeedStats> {
    let solution = repo.join("solution");
    anyhow::ensure!(
        solution.is_dir(),
        "{} 缺少 solution/ 目录",
        repo.display()
    );

    let mut stats = SeedStats::default();
    let mut problems: Vec<ParsedProblem> = Vec::with_capacity(4096);

    // 遍历 solution/<bucket>/<题目目录>/
    let mut bucket_dirs: Vec<PathBuf> = std::fs::read_dir(&solution)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.path())
        .collect();
    bucket_dirs.sort();

    for bucket in bucket_dirs {
        let mut prob_dirs: Vec<PathBuf> = std::fs::read_dir(&bucket)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .map(|e| e.path())
            .collect();
        prob_dirs.sort();

        for dir in prob_dirs {
            if !dir.join("README.md").is_file() {
                stats.skipped_no_readme += 1;
                continue;
            }
            match parse_problem(repo, &dir) {
                Some(p) => problems.push(p),
                None => stats.skipped_parse_failed += 1,
            }
        }
    }

    // 汇总所有出现的中文 tag → 英文 (slug, name_en)
    let mut tag_meta: HashMap<String, (String, String)> = HashMap::new();
    for p in &problems {
        for cn in &p.tags_cn {
            let (slug, en) = cn_tag_to_meta(cn);
            tag_meta
                .entry(cn.clone())
                .or_insert((slug, en));
        }
    }
    stats.tags = tag_meta.len();

    // 事务写库
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM problems_fts").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM problem_tags").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM problems").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM tags").execute(&mut *tx).await?;

    let mut tag_id: HashMap<String, i64> = HashMap::new();
    for (cn, (slug, en)) in &tag_meta {
        let res = sqlx::query("INSERT INTO tags (slug, name_en, name_cn) VALUES (?, ?, ?)")
            .bind(slug)
            .bind(en)
            .bind(cn)
            .execute(&mut *tx)
            .await?;
        tag_id.insert(cn.clone(), res.last_insert_rowid());
    }

    for p in &problems {
        sqlx::query(
            "INSERT INTO problems
                (id, slug, title_en, title_cn, difficulty, is_premium,
                 leetcode_url, leetcode_cn_url, acceptance_rate, repo_dir)
             VALUES (?, ?, ?, ?, ?, 0, ?, ?, NULL, ?)",
        )
        .bind(p.id)
        .bind(&p.slug)
        .bind(&p.title_en)
        .bind(&p.title_cn)
        .bind(&p.difficulty)
        .bind(&p.leetcode_url)
        .bind(&p.leetcode_cn_url)
        .bind(&p.repo_dir)
        .execute(&mut *tx)
        .await?;

        for cn in &p.tags_cn {
            if let Some(tid) = tag_id.get(cn) {
                sqlx::query(
                    "INSERT OR IGNORE INTO problem_tags (problem_id, tag_id) VALUES (?, ?)",
                )
                .bind(p.id)
                .bind(tid)
                .execute(&mut *tx)
                .await?;
            }
        }

        // FTS：tags blob 拼接 slug + 中英名，便于按 tag 文本搜
        let tags_blob = p
            .tags_cn
            .iter()
            .filter_map(|cn| tag_meta.get(cn).map(|(slug, en)| format!("{} {} {}", slug, en, cn)))
            .collect::<Vec<_>>()
            .join(" ");
        sqlx::query(
            "INSERT INTO problems_fts (id, title_en, title_cn, slug, tags) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(p.id)
        .bind(&p.title_en)
        .bind(&p.title_cn)
        .bind(&p.slug)
        .bind(&tags_blob)
        .execute(&mut *tx)
        .await?;

        stats.problems += 1;
    }

    tx.commit().await?;
    Ok(stats)
}

// -------------------- 解析层 --------------------

#[derive(Debug)]
struct ParsedProblem {
    id: i64,
    slug: String,
    title_cn: String,
    title_en: String,
    difficulty: String,
    leetcode_url: String,
    leetcode_cn_url: String,
    tags_cn: Vec<String>,
    repo_dir: String,
}

fn parse_problem(repo: &Path, dir: &Path) -> Option<ParsedProblem> {
    let readme = dir.join("README.md");
    let raw = std::fs::read_to_string(&readme).ok()?;
    let (front, body) = split_front_matter(&raw);
    let front = front?;

    let difficulty = parse_difficulty(front).unwrap_or_else(|| "Medium".to_string());
    let tags_cn = parse_tags(front);

    let h1 = first_h1(body)?;
    let (id, title_cn, slug, leetcode_cn_url) = parse_h1_line(&h1)?;

    let (title_en, leetcode_url) = parse_en_h1(dir).unwrap_or_else(|| {
        let fallback = dir
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|n| n.split_once('.'))
            .map(|(_, t)| t.trim().to_string())
            .unwrap_or_else(|| title_cn.clone());
        let url = format!("https://leetcode.com/problems/{}/", slug);
        (fallback, url)
    });

    let repo_dir = dir
        .strip_prefix(repo)
        .ok()?
        .to_string_lossy()
        .replace('\\', "/");

    Some(ParsedProblem {
        id,
        slug,
        title_cn,
        title_en,
        difficulty,
        leetcode_url,
        leetcode_cn_url,
        tags_cn,
        repo_dir,
    })
}

fn split_front_matter(s: &str) -> (Option<&str>, &str) {
    let s = s.trim_start_matches('\u{feff}');
    if let Some(rest) = s.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            return (Some(&rest[..end]), &rest[end + 5..]);
        }
    }
    (None, s)
}

fn parse_difficulty(front: &str) -> Option<String> {
    let line = front
        .lines()
        .find(|l| l.trim_start().starts_with("difficulty:"))?;
    let val = line.split_once(':')?.1.trim();
    Some(match val {
        "简单" | "Easy" => "Easy".into(),
        "中等" | "Medium" => "Medium".into(),
        "困难" | "Hard" => "Hard".into(),
        other => other.to_string(),
    })
}

fn parse_tags(front: &str) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    let mut in_tags = false;
    for line in front.lines() {
        let trimmed = line.trim_start();
        let indented = line.starts_with(' ') || line.starts_with('\t');
        if !indented {
            // 顶层键
            in_tags = trimmed.starts_with("tags:");
            continue;
        }
        if in_tags {
            if let Some(rest) = trimmed.strip_prefix("- ") {
                let v = rest.trim().trim_matches('"').trim_matches('\'').to_string();
                if !v.is_empty() {
                    tags.push(v);
                }
            }
        }
    }
    tags
}

fn first_h1(body: &str) -> Option<String> {
    for line in body.lines() {
        let t = line.trim_start();
        if let Some(rest) = t.strip_prefix("# ") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// 解析 H1 形如 `[1. 两数之和](https://leetcode.cn/problems/two-sum)`
/// 返回 (id, title, slug, leetcode_cn_url)
fn parse_h1_line(h1: &str) -> Option<(i64, String, String, String)> {
    let h1 = h1.trim();
    let inside = h1.strip_prefix('[')?;
    let bracket_end = inside.find("](")?;
    let inner = &inside[..bracket_end];
    let url_part = &inside[bracket_end + 2..];
    let url = url_part.strip_suffix(')')?.trim();

    let (id_str, title) = inner.split_once('.')?;
    let id: i64 = id_str.trim().parse().ok()?;
    let title = title.trim().to_string();

    let slug_part = url.strip_prefix("https://leetcode.cn/problems/")?;
    let slug = slug_part.trim_end_matches('/').to_string();
    if slug.is_empty() {
        return None;
    }

    let canonical_url = format!("https://leetcode.cn/problems/{}/", slug);
    Some((id, title, slug, canonical_url))
}

fn parse_en_h1(dir: &Path) -> Option<(String, String)> {
    let raw = std::fs::read_to_string(dir.join("README_EN.md")).ok()?;
    let (_, body) = split_front_matter(&raw);
    let h1 = first_h1(body)?;
    let h1 = h1.trim();
    let inside = h1.strip_prefix('[')?;
    let bracket_end = inside.find("](")?;
    let inner = &inside[..bracket_end];
    let url_part = &inside[bracket_end + 2..];
    let url = url_part.strip_suffix(')')?.trim();

    let (_id, title) = inner.split_once('.')?;
    let title = title.trim().to_string();

    // 英文 README 的链接是 https://leetcode.com/problems/<slug>
    let leetcode_url = if let Some(rest) = url.strip_prefix("https://leetcode.com/problems/") {
        let slug = rest.trim_end_matches('/');
        format!("https://leetcode.com/problems/{}/", slug)
    } else {
        url.to_string()
    };

    Some((title, leetcode_url))
}

// -------------------- 中文 tag → (slug, en_name) 映射 --------------------

const TAG_MAP: &[(&str, &str, &str)] = &[
    ("数组", "array", "Array"),
    ("字符串", "string", "String"),
    ("哈希表", "hash-table", "Hash Table"),
    ("数学", "math", "Math"),
    ("动态规划", "dynamic-programming", "Dynamic Programming"),
    ("排序", "sorting", "Sorting"),
    ("贪心", "greedy", "Greedy"),
    ("深度优先搜索", "depth-first-search", "Depth-First Search"),
    ("二分查找", "binary-search", "Binary Search"),
    ("数据库", "database", "Database"),
    ("位运算", "bit-manipulation", "Bit Manipulation"),
    ("矩阵", "matrix", "Matrix"),
    ("树", "tree", "Tree"),
    ("广度优先搜索", "breadth-first-search", "Breadth-First Search"),
    ("双指针", "two-pointers", "Two Pointers"),
    ("前缀和", "prefix-sum", "Prefix Sum"),
    ("堆（优先队列）", "heap-priority-queue", "Heap (Priority Queue)"),
    ("模拟", "simulation", "Simulation"),
    ("计数", "counting", "Counting"),
    ("二叉树", "binary-tree", "Binary Tree"),
    ("图", "graph", "Graph"),
    ("栈", "stack", "Stack"),
    ("滑动窗口", "sliding-window", "Sliding Window"),
    ("枚举", "enumeration", "Enumeration"),
    ("设计", "design", "Design"),
    ("回溯", "backtracking", "Backtracking"),
    ("并查集", "union-find", "Union Find"),
    ("数论", "number-theory", "Number Theory"),
    ("链表", "linked-list", "Linked List"),
    ("有序集合", "ordered-set", "Ordered Set"),
    ("线段树", "segment-tree", "Segment Tree"),
    ("单调栈", "monotonic-stack", "Monotonic Stack"),
    ("JavaScript", "javascript", "JavaScript"),
    ("分治", "divide-and-conquer", "Divide and Conquer"),
    ("字典树", "trie", "Trie"),
    ("组合数学", "combinatorics", "Combinatorics"),
    ("位掩码", "bitmask", "Bitmask"),
    ("队列", "queue", "Queue"),
    ("递归", "recursion", "Recursion"),
    ("几何", "geometry", "Geometry"),
    ("树状数组", "binary-indexed-tree", "Binary Indexed Tree"),
    ("记忆化", "memoization", "Memoization"),
    ("二叉搜索树", "binary-search-tree", "Binary Search Tree"),
    ("哈希函数", "hash-function", "Hash Function"),
    ("拓扑排序", "topological-sort", "Topological Sort"),
    ("最短路", "shortest-path", "Shortest Path"),
    ("字符串匹配", "string-matching", "String Matching"),
    ("滚动哈希", "rolling-hash", "Rolling Hash"),
    ("博弈", "game-theory", "Game Theory"),
    ("交互", "interactive", "Interactive"),
    ("数据流", "data-stream", "Data Stream"),
    ("单调队列", "monotonic-queue", "Monotonic Queue"),
    ("脑筋急转弯", "brainteaser", "Brainteaser"),
    ("归并排序", "merge-sort", "Merge Sort"),
    ("双向链表", "doubly-linked-list", "Doubly-Linked List"),
    ("Pandas", "pandas", "Pandas"),
    ("随机化", "randomized", "Randomized"),
    ("计数排序", "counting-sort", "Counting Sort"),
    ("迭代器", "iterator", "Iterator"),
    ("多线程", "concurrency", "Concurrency"),
    ("扫描线", "line-sweep", "Line Sweep"),
    ("快速选择", "quickselect", "Quickselect"),
    ("后缀数组", "suffix-array", "Suffix Array"),
    ("概率与统计", "probability-and-statistics", "Probability and Statistics"),
    ("最小生成树", "minimum-spanning-tree", "Minimum Spanning Tree"),
    ("桶排序", "bucket-sort", "Bucket Sort"),
    ("水塘抽样", "reservoir-sampling", "Reservoir Sampling"),
    ("Shell", "shell", "Shell"),
    ("欧拉回路", "eulerian-circuit", "Eulerian Circuit"),
    ("基数排序", "radix-sort", "Radix Sort"),
    ("拒绝采样", "rejection-sampling", "Rejection Sampling"),
    ("强连通分量", "strongly-connected-component", "Strongly Connected Component"),
    ("双连通分量", "biconnected-component", "Biconnected Component"),
];

fn cn_tag_to_meta(cn: &str) -> (String, String) {
    for (k, slug, en) in TAG_MAP {
        if *k == cn {
            return ((*slug).to_string(), (*en).to_string());
        }
    }
    // 兜底：未识别的中文 tag 直接用其本身作 slug + 英文名
    (cn.to_string(), cn.to_string())
}

// -------------------- 题面文件读取（运行时由 routes::problems::statement 调用） --------------------

/// 给定题目 repo_dir，读 README.md 或 README_EN.md，剥掉 YAML front matter 返回 markdown 正文。
/// 校验路径不含 `..`，确保停留在 repo 子目录里。
pub fn read_statement(repo: &Path, repo_dir: &str, lang: Lang) -> Result<String> {
    anyhow::ensure!(
        !repo_dir.is_empty() && !repo_dir.contains("..") && !repo_dir.starts_with('/'),
        "非法 repo_dir: {repo_dir}"
    );
    let filename = match lang {
        Lang::Cn => "README.md",
        Lang::En => "README_EN.md",
    };
    let path = repo.join(repo_dir).join(filename);
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("读取题面失败: {}", path.display()))?;
    let (_, body) = split_front_matter(&raw);
    Ok(body.trim_start().to_string())
}

#[derive(Debug, Clone, Copy)]
pub enum Lang {
    Cn,
    En,
}
