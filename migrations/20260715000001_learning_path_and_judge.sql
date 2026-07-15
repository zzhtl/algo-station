-- 渐进式课程、间隔复习与本地判题队列。单用户、离线优先。

CREATE TABLE lesson_progress (
    lesson_slug         TEXT PRIMARY KEY,
    status              TEXT NOT NULL DEFAULT 'not_started'
                        CHECK (status IN ('not_started', 'in_progress', 'completed')),
    animation_completed INTEGER NOT NULL DEFAULT 0 CHECK (animation_completed IN (0, 1)),
    quiz_best_score     INTEGER NOT NULL DEFAULT 0 CHECK (quiz_best_score BETWEEN 0 AND 100),
    note                TEXT NOT NULL DEFAULT '',
    completed_at        TEXT,
    updated_at          TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE quiz_attempts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    lesson_slug TEXT NOT NULL,
    score       INTEGER NOT NULL CHECK (score BETWEEN 0 AND 100),
    total       INTEGER NOT NULL CHECK (total > 0),
    answers     TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_quiz_attempts_lesson_created
    ON quiz_attempts (lesson_slug, created_at DESC);

CREATE TABLE review_schedules (
    lesson_slug TEXT PRIMARY KEY,
    step        INTEGER NOT NULL DEFAULT 0 CHECK (step BETWEEN 0 AND 4),
    due_at      TEXT NOT NULL,
    last_rating TEXT CHECK (last_rating IN ('forgotten', 'fuzzy', 'remembered')),
    mastered    INTEGER NOT NULL DEFAULT 0 CHECK (mastered IN (0, 1)),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_review_schedules_due ON review_schedules (mastered, due_at);

CREATE TABLE review_attempts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    lesson_slug TEXT NOT NULL,
    rating      TEXT NOT NULL CHECK (rating IN ('forgotten', 'fuzzy', 'remembered')),
    previous_step INTEGER NOT NULL,
    next_step   INTEGER NOT NULL,
    next_due_at TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_review_attempts_lesson_created
    ON review_attempts (lesson_slug, created_at DESC);

CREATE TABLE exercise_drafts (
    exercise_slug TEXT NOT NULL,
    language      TEXT NOT NULL CHECK (language IN ('go', 'rust')),
    contract      TEXT NOT NULL CHECK (contract IN ('function', 'stdio')),
    code          TEXT NOT NULL DEFAULT '',
    updated_at    TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (exercise_slug, language, contract)
);

CREATE TABLE submissions (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    exercise_slug  TEXT NOT NULL,
    lesson_slug    TEXT NOT NULL,
    language       TEXT NOT NULL CHECK (language IN ('go', 'rust')),
    contract       TEXT NOT NULL CHECK (contract IN ('function', 'stdio')),
    source_code    TEXT NOT NULL,
    status         TEXT NOT NULL DEFAULT 'queued'
                   CHECK (status IN (
                       'queued', 'running', 'accepted', 'wrong_answer',
                       'compile_error', 'runtime_error', 'time_limit',
                       'memory_limit', 'internal_error'
                   )),
    attempts       INTEGER NOT NULL DEFAULT 0 CHECK (attempts BETWEEN 0 AND 3),
    lease_owner    TEXT,
    lease_until    TEXT,
    result_json    TEXT,
    queued_at      TEXT NOT NULL DEFAULT (datetime('now')),
    started_at     TEXT,
    finished_at    TEXT,
    updated_at     TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_submissions_queue ON submissions (status, queued_at, id);
CREATE INDEX idx_submissions_exercise ON submissions (exercise_slug, id DESC);
CREATE INDEX idx_submissions_lesson_status ON submissions (lesson_slug, status);

CREATE TABLE learning_events (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type    TEXT NOT NULL,
    lesson_slug   TEXT,
    exercise_slug TEXT,
    event_date    TEXT NOT NULL DEFAULT (date('now')),
    created_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_learning_events_date ON learning_events (event_date, event_type);

CREATE TABLE daily_plan_overrides (
    plan_date      TEXT PRIMARY KEY,
    target_minutes INTEGER NOT NULL CHECK (target_minutes BETWEEN 10 AND 240),
    updated_at     TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE judge_workers (
    worker_id        TEXT PRIMARY KEY,
    version          TEXT NOT NULL,
    status           TEXT NOT NULL CHECK (status IN ('online', 'draining', 'offline')),
    current_submission_id INTEGER,
    last_heartbeat_at TEXT NOT NULL,
    started_at       TEXT NOT NULL DEFAULT (datetime('now'))
);

