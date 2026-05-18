-- 题目对应在 data/leetcode/ 内的相对目录，例如：
--   solution/0000-0099/0001.Two Sum
-- 用于运行时定位 README.md / README_EN.md 文件。
ALTER TABLE problems ADD COLUMN repo_dir TEXT NOT NULL DEFAULT '';
