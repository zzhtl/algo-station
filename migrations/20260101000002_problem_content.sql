-- 题面内容（中英文）+ 题解讲解。来源标注用于 CC-BY-SA 4.0 署名义务。
CREATE TABLE problem_statements (
    problem_id     INTEGER PRIMARY KEY REFERENCES problems(id) ON DELETE CASCADE,
    statement_cn   TEXT NOT NULL DEFAULT '',
    statement_en   TEXT NOT NULL DEFAULT '',
    explanation_cn TEXT NOT NULL DEFAULT '',
    source         TEXT NOT NULL DEFAULT 'doocs/leetcode',
    source_url     TEXT NOT NULL DEFAULT '',
    license        TEXT NOT NULL DEFAULT 'CC-BY-SA-4.0',
    updated_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 不同语言的参考题解代码（当前覆盖 Go / Rust）。
CREATE TABLE problem_solutions (
    problem_id INTEGER NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
    lang       TEXT NOT NULL,
    code       TEXT NOT NULL,
    source     TEXT NOT NULL DEFAULT 'doocs/leetcode',
    source_url TEXT NOT NULL DEFAULT '',
    license    TEXT NOT NULL DEFAULT 'CC-BY-SA-4.0',
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (problem_id, lang)
);

CREATE INDEX idx_problem_solutions_lang ON problem_solutions(lang);
