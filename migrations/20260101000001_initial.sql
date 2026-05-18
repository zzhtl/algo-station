-- 题目元数据。仅存储事实信息（题号/题名/难度/标签/链接），
-- 不存储 LeetCode 的题面文本。详细讲解放在 articles 表里，由站点作者原创撰写。
CREATE TABLE problems (
    id              INTEGER PRIMARY KEY,           -- LeetCode 题号
    slug            TEXT NOT NULL UNIQUE,          -- 英文 slug，例如 "two-sum"
    title_en        TEXT NOT NULL,
    title_cn        TEXT NOT NULL,
    difficulty      TEXT NOT NULL CHECK (difficulty IN ('Easy', 'Medium', 'Hard')),
    is_premium      INTEGER NOT NULL DEFAULT 0,
    leetcode_url    TEXT NOT NULL,
    leetcode_cn_url TEXT NOT NULL,
    acceptance_rate REAL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_problems_difficulty ON problems(difficulty);
CREATE INDEX idx_problems_slug ON problems(slug);

CREATE TABLE tags (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    slug    TEXT NOT NULL UNIQUE,                  -- 例如 "array"
    name_en TEXT NOT NULL,
    name_cn TEXT NOT NULL
);

CREATE TABLE problem_tags (
    problem_id INTEGER NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
    tag_id     INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (problem_id, tag_id)
);

CREATE INDEX idx_problem_tags_tag ON problem_tags(tag_id);

-- 原创题解文章。Markdown 源在文件系统 content/articles/{slug}.md，
-- 此表只索引元数据用于列表和搜索。
CREATE TABLE articles (
    slug         TEXT PRIMARY KEY,
    title        TEXT NOT NULL,
    category     TEXT NOT NULL,                    -- 数组/链表/树/DP/...
    summary      TEXT NOT NULL,
    problem_ids  TEXT NOT NULL DEFAULT '[]',       -- JSON 数组，关联的题号
    order_in_cat INTEGER NOT NULL DEFAULT 0,
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_articles_category ON articles(category);

-- 全文搜索：覆盖题号、题名、标签
CREATE VIRTUAL TABLE problems_fts USING fts5(
    id UNINDEXED,
    title_en,
    title_cn,
    slug,
    tags,
    tokenize='unicode61'
);
