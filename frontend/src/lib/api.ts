export type Difficulty = 'Easy' | 'Medium' | 'Hard';

export interface TagBrief {
  slug: string;
  name_en: string;
  name_cn: string;
}

export interface TagFull extends TagBrief {
  id: number;
  problem_count: number;
}

export interface ProblemListItem {
  id: number;
  slug: string;
  title_en: string;
  title_cn: string;
  difficulty: Difficulty;
  is_premium: boolean;
  leetcode_url: string;
  leetcode_cn_url: string;
  acceptance_rate: number | null;
  tags: TagBrief[];
  has_article: boolean;
}

export interface ProblemDetail extends ProblemListItem {
  related_articles: {
    slug: string;
    title: string;
    category: string;
    summary: string;
  }[];
  has_statement: boolean;
  statement_source: string | null;
  statement_source_url: string | null;
  statement_license: string | null;
}

export type StatementLang = 'cn' | 'en';

export interface StatementResponse {
  problem_id: number;
  lang: StatementLang;
  content: string;
  source: string;
  source_url: string;
  license: string;
}

export interface ArticleListItem {
  slug: string;
  title: string;
  category: string;
  difficulty: Difficulty;
  summary: string;
  problem_ids: number[];
  order_in_cat: number;
}

export interface ArticleFull extends ArticleListItem {
  content: string;
}

export interface Pagination<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
}

export interface Stats {
  total_problems: number;
  easy: number;
  medium: number;
  hard: number;
  total_tags: number;
  total_articles: number;
}

const BASE = '/api';

async function get<T>(fetchFn: typeof fetch, path: string): Promise<T> {
  const res = await fetchFn(`${BASE}${path}`);
  if (!res.ok) {
    throw new Error(`API ${res.status}: ${path}`);
  }
  return res.json() as Promise<T>;
}

export const api = {
  stats: (f = fetch) => get<Stats>(f, '/stats'),
  tags: (f = fetch) => get<TagFull[]>(f, '/tags'),
  problems: (params: Record<string, string | number | boolean | undefined>, f = fetch) => {
    const usp = new URLSearchParams();
    for (const [k, v] of Object.entries(params)) {
      if (v === undefined || v === '' || v === null) continue;
      usp.set(k, String(v));
    }
    const qs = usp.toString();
    return get<Pagination<ProblemListItem>>(f, `/problems${qs ? '?' + qs : ''}`);
  },
  problem: (id: number | string, f = fetch) => get<ProblemDetail>(f, `/problems/${id}`),
  statement: (id: number | string, lang: StatementLang = 'cn', f = fetch) =>
    get<StatementResponse>(f, `/problems/${id}/statement?lang=${lang}`),
  articles: (f = fetch) => get<ArticleListItem[]>(f, '/articles'),
  article: (slug: string, f = fetch) => get<ArticleFull>(f, `/articles/${slug}`)
};
