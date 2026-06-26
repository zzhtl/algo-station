-- 用 trigram 分词器重建题目全文索引：支持中文子串匹配（≥3 字符）+ 标签文本检索。
-- 原 unicode61 对中文几乎无效，搜索一直走 LIKE 全表扫描。
DROP TABLE IF EXISTS problems_fts;

CREATE VIRTUAL TABLE problems_fts USING fts5(
    id UNINDEXED,
    title_en,
    title_cn,
    slug,
    tags,
    tokenize='trigram'
);

-- 从已有 problems/tags 回填。全新库此时 problems 为空 → 空操作，随后由 seed 填充；
-- 已有数据的库（data/algo.db）则一次性把索引补齐，无需重跑 import。
-- tags blob 格式与 problem_seed.rs 保持一致："slug en cn" 以空格连接。
INSERT INTO problems_fts (id, title_en, title_cn, slug, tags)
SELECT
    p.id,
    p.title_en,
    p.title_cn,
    p.slug,
    COALESCE(
        (
            SELECT group_concat(t.slug || ' ' || t.name_en || ' ' || t.name_cn, ' ')
            FROM problem_tags pt
            JOIN tags t ON t.id = pt.tag_id
            WHERE pt.problem_id = p.id
        ),
        ''
    )
FROM problems p;
