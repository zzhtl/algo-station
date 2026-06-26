-- 进度 / 草稿 / 收藏 后端持久化。完全离线、单用户，无鉴权。
-- 之前这些数据只存在浏览器 localStorage，换浏览器即丢；落库后同机跨浏览器不丢、可导出。

-- 训练进度：按原创题解 slug 记录一条。状态由前端推导后写入。
CREATE TABLE training_records (
    article_slug       TEXT PRIMARY KEY,
    status             TEXT NOT NULL DEFAULT 'todo',  -- todo | learned | practiced | reviewed
    pattern_note       TEXT NOT NULL DEFAULT '',
    completed_problems TEXT NOT NULL DEFAULT '[]',    -- JSON 数组，已完成题号
    attempt_result     TEXT NOT NULL DEFAULT '',      -- '' | independent | hinted | failed
    stuck_note         TEXT NOT NULL DEFAULT '',
    review_note        TEXT NOT NULL DEFAULT '',
    updated_at         TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 练习代码草稿：按题号 + 语言。
CREATE TABLE practice_drafts (
    problem_id INTEGER NOT NULL,
    lang       TEXT NOT NULL,
    code       TEXT NOT NULL DEFAULT '',
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (problem_id, lang)
);

-- 题目收藏。
CREATE TABLE bookmarks (
    problem_id INTEGER PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
