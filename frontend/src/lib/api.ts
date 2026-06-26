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

export interface SolutionCode {
  lang: string;
  label: string;
  code: string;
}

export interface SolutionsResponse {
  problem_id: number;
  solutions: SolutionCode[];
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

export interface ApiTrainingRecord {
  article_slug: string;
  status: string;
  pattern_note: string;
  completed_problems: number[];
  attempt_result: string;
  stuck_note: string;
  review_note: string;
  updated_at?: string;
}

export type TrainingInput = Omit<ApiTrainingRecord, 'article_slug' | 'updated_at'>;

export interface Draft {
  problem_id: number;
  lang: string;
  code: string;
  updated_at?: string;
}

export interface CategoryProgress {
  category: string;
  total_articles: number;
  learned: number;
  practiced: number;
  reviewed: number;
}

export interface ProgressStats {
  categories: CategoryProgress[];
  total_articles: number;
  learned: number;
  practiced: number;
  reviewed: number;
  todo: number;
}

export interface ProgressExport {
  training: ApiTrainingRecord[];
  drafts: Draft[];
  bookmarks: number[];
  exported_at?: string;
}

const BASE = '/api';

async function get<T>(fetchFn: typeof fetch, path: string): Promise<T> {
  const res = await fetchFn(`${BASE}${path}`);
  if (!res.ok) {
    throw new Error(`API ${res.status}: ${path}`);
  }
  return res.json() as Promise<T>;
}

async function send<T>(
  fetchFn: typeof fetch,
  method: string,
  path: string,
  body?: unknown
): Promise<T> {
  const res = await fetchFn(`${BASE}${path}`, {
    method,
    headers: body !== undefined ? { 'content-type': 'application/json' } : undefined,
    body: body !== undefined ? JSON.stringify(body) : undefined
  });
  if (!res.ok) throw new Error(`API ${res.status}: ${path}`);
  const text = await res.text();
  return (text ? JSON.parse(text) : undefined) as T;
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
  solutions: (id: number | string, lang?: string, f = fetch) =>
    get<SolutionsResponse>(
      f,
      `/problems/${id}/solutions${lang ? '?lang=' + encodeURIComponent(lang) : ''}`
    ),
  articles: (f = fetch) => get<ArticleListItem[]>(f, '/articles'),
  article: (slug: string, f = fetch) => get<ArticleFull>(f, `/articles/${slug}`),

  // 进度 / 草稿 / 收藏（后端持久化，跨浏览器不丢）
  trainingRecords: (f = fetch) => get<ApiTrainingRecord[]>(f, '/progress/training'),
  putTraining: (slug: string, body: TrainingInput, f = fetch) =>
    send<ApiTrainingRecord>(f, 'PUT', `/progress/training/${encodeURIComponent(slug)}`, body),
  drafts: (id: number | string, f = fetch) => get<Draft[]>(f, `/progress/drafts/${id}`),
  putDraft: (id: number | string, lang: string, code: string, f = fetch) =>
    send<Draft>(f, 'PUT', `/progress/drafts/${id}/${lang}`, { code }),
  bookmarks: (f = fetch) => get<number[]>(f, '/bookmarks'),
  addBookmark: (id: number | string, f = fetch) => send<void>(f, 'PUT', `/bookmarks/${id}`),
  removeBookmark: (id: number | string, f = fetch) => send<void>(f, 'DELETE', `/bookmarks/${id}`),
  progressStats: (f = fetch) => get<ProgressStats>(f, '/stats/progress'),
  exportProgress: (f = fetch) => get<ProgressExport>(f, '/progress/export'),
  importProgress: (body: ProgressExport, f = fetch) =>
    send<{ training: number; drafts: number; bookmarks: number }>(f, 'POST', '/progress/import', body)
};
