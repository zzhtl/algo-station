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
  schema_version?: number;
  training: ApiTrainingRecord[];
  drafts: Draft[];
  bookmarks: number[];
  lessons?: LessonProgressExport[];
  exercise_drafts?: ExerciseDraftExport[];
  reviews?: ReviewScheduleExport[];
  exported_at?: string;
}

export type LessonStatus = 'not_started' | 'in_progress' | 'completed';
export type Language = 'go' | 'rust';
export type ExerciseContract = 'function' | 'stdio';
export type ReviewRating = 'forgotten' | 'fuzzy' | 'remembered';

export interface LessonProgressExport {
  lesson_slug: string;
  status: LessonStatus;
  animation_completed: boolean;
  quiz_best_score: number;
  note: string;
  completed_at: string | null;
}

export interface ExerciseDraftExport {
  exercise_slug: string;
  language: Language;
  contract: ExerciseContract;
  code: string;
}

export interface ReviewScheduleExport {
  lesson_slug: string;
  step: number;
  due_at: string;
  last_rating: ReviewRating | null;
  mastered: boolean;
}

export interface ProgressView {
  lesson_slug: string;
  status: LessonStatus;
  animation_completed: boolean;
  quiz_best_score: number;
  core_exercise_accepted: boolean;
  note: string;
  completed_at: string | null;
  updated_at: string | null;
}

export interface LessonCard {
  slug: string;
  stage_id: string;
  title: string;
  summary: string;
  order: number;
  estimated_minutes: number;
  status: LessonStatus;
  prerequisites_met: boolean;
  has_visualization: boolean;
  exercise_count: number;
}

export interface StageView {
  id: string;
  title: string;
  description: string;
  order: number;
  completed_lessons: number;
  lessons: LessonCard[];
}

export interface CurriculumResponse {
  summary: {
    stage_count: number;
    lesson_count: number;
    exercise_count: number;
    visualization_count: number;
    completed_lessons: number;
    completion_percent: number;
  };
  stages: StageView[];
}

export interface DailyPlan {
  date: string;
  target_minutes: number;
  estimated_minutes: number;
  lessons: LessonCard[];
}

export interface DashboardResponse {
  completed_lessons: number;
  total_lessons: number;
  completion_percent: number;
  streak_days: number;
  due_reviews: number;
  next_lesson: LessonCard | null;
  daily_plan: DailyPlan;
  judge_online: boolean;
}

export interface PublicQuestion {
  id: string;
  prompt: string;
  options: string[];
}

export interface LessonPublic {
  slug: string;
  stage_id: string;
  article_slug: string;
  title: string;
  summary: string;
  order: number;
  estimated_minutes: number;
  prerequisites: string[];
  objectives: string[];
  quiz: PublicQuestion[];
}

export interface VisualizationMeta {
  id: string;
  lesson_slug: string;
  title: string;
  kind: string;
  description: string;
}

export interface ExerciseSummary {
  slug: string;
  problem_id: number;
  title: string;
  difficulty: Difficulty;
  summary: string;
  core: boolean;
  accepted: boolean;
}

export interface LessonDetailResponse {
  lesson: LessonPublic;
  progress: ProgressView;
  prerequisites_met: boolean;
  visualization: VisualizationMeta | null;
  exercises: ExerciseSummary[];
}

export interface QuizAttemptResponse {
  score: number;
  passed: boolean;
  corrections: {
    question_id: string;
    selected_index: number;
    correct_index: number;
    correct: boolean;
    explanation: string;
  }[];
  progress: ProgressView;
}

export interface ReviewItem {
  lesson_slug: string;
  title: string;
  step: number;
  due_at: string;
  last_rating: ReviewRating | null;
  mastered: boolean;
  due: boolean;
}

export interface ExerciseListItem {
  slug: string;
  lesson_slug: string;
  stage_id: string;
  problem_id: number;
  title: string;
  difficulty: Difficulty;
  summary: string;
  accepted: boolean;
}

export interface ExerciseListResponse {
  items: ExerciseListItem[];
  next_cursor: number | null;
  total: number;
}

export interface StarterTemplate {
  language: Language;
  contract: ExerciseContract;
  code: string;
}

export interface ExerciseLimits {
  compile_ms: number;
  case_ms: number;
  total_ms: number;
  memory_mb: number;
  output_kb: number;
}

export interface ExerciseDetailResponse {
  slug: string;
  problem_id: number;
  lesson_slug: string;
  title: string;
  difficulty: Difficulty;
  summary: string;
  starters: StarterTemplate[];
  public_cases: { name: string; input: string; expected: string }[];
  hidden_case_count: number;
  limits: ExerciseLimits;
  accepted: boolean;
}

export interface ExerciseDraft {
  exercise_slug: string;
  language: Language;
  contract: ExerciseContract;
  code: string;
  updated_at: string;
}

export type FinishedSubmissionStatus =
  | 'accepted'
  | 'wrong_answer'
  | 'compile_error'
  | 'runtime_error'
  | 'time_limit'
  | 'memory_limit'
  | 'internal_error';
export type SubmissionStatus = 'queued' | 'running' | FinishedSubmissionStatus;

export interface JudgeCaseResult {
  name: string;
  visibility: 'public' | 'hidden';
  status: FinishedSubmissionStatus;
  duration_ms: number;
  actual?: string;
  message?: string;
}

export interface JudgeResult {
  status: FinishedSubmissionStatus;
  message: string;
  compile_output: string;
  cases: JudgeCaseResult[];
  duration_ms: number;
}

interface SubmissionBase {
  id: number;
  exercise_slug: string;
  lesson_slug: string;
  language: Language;
  contract: ExerciseContract;
  source_code: string;
  attempts: number;
  queued_at: string;
  started_at: string | null;
  finished_at: string | null;
  updated_at: string;
}

export type Submission =
  | (SubmissionBase & { status: 'queued' | 'running'; result: null })
  | (SubmissionBase & { status: FinishedSubmissionStatus; result: JudgeResult | null });

export interface SubmissionListResponse {
  items: Submission[];
  next_cursor: number | null;
}

export interface JudgeStatusResponse {
  online: boolean;
  queue_size: number;
  running: number;
  last_heartbeat_at: string | null;
}

export interface ProblemDetails {
  type: string;
  title: string;
  status: number;
  detail: string;
  error: string;
}

export class ApiRequestError extends Error {
  constructor(
    public status: number,
    public problem: ProblemDetails,
    public path: string
  ) {
    super(problem.detail || `API ${status}: ${path}`);
    this.name = 'ApiRequestError';
  }
}

const BASE = '/api';

async function get<T>(fetchFn: typeof fetch, path: string): Promise<T> {
  const res = await fetchFn(`${BASE}${path}`);
  if (!res.ok) {
    throw await apiError(res, path);
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
  if (!res.ok) throw await apiError(res, path);
  const text = await res.text();
  return (text ? JSON.parse(text) : undefined) as T;
}

async function apiError(response: Response, path: string): Promise<ApiRequestError> {
  const fallback: ProblemDetails = {
    type: 'about:blank',
    title: response.statusText || 'Request failed',
    status: response.status,
    detail: `API ${response.status}: ${path}`,
    error: `API ${response.status}: ${path}`
  };
  try {
    const value = (await response.json()) as Partial<ProblemDetails>;
    return new ApiRequestError(response.status, { ...fallback, ...value }, path);
  } catch {
    return new ApiRequestError(response.status, fallback, path);
  }
}

function queryString(params: Record<string, string | number | boolean | undefined | null>) {
  const search = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value === undefined || value === null || value === '') continue;
    search.set(key, String(value));
  }
  const value = search.toString();
  return value ? `?${value}` : '';
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

  dashboard: (f = fetch) => get<DashboardResponse>(f, '/dashboard'),
  curriculum: (f = fetch) => get<CurriculumResponse>(f, '/curriculum'),
  lesson: (slug: string, f = fetch) =>
    get<LessonDetailResponse>(f, `/lessons/${encodeURIComponent(slug)}`),
  patchLessonProgress: (
    slug: string,
    body: { animation_completed?: boolean; note?: string },
    f = fetch
  ) => send<ProgressView>(f, 'PATCH', `/lessons/${encodeURIComponent(slug)}/progress`, body),
  submitQuiz: (slug: string, answers: number[], f = fetch) =>
    send<QuizAttemptResponse>(
      f,
      'POST',
      `/lessons/${encodeURIComponent(slug)}/quiz-attempts`,
      { answers }
    ),
  dailyPlan: (date?: string, f = fetch) =>
    get<DailyPlan>(f, `/daily-plan${queryString({ date })}`),
  putDailyPlan: (target_minutes: number, date?: string, f = fetch) =>
    send<DailyPlan>(f, 'PUT', '/daily-plan', { date, target_minutes }),
  reviews: (f = fetch) => get<ReviewItem[]>(f, '/reviews'),
  submitReview: (slug: string, rating: ReviewRating, f = fetch) =>
    send<ReviewItem>(f, 'POST', `/reviews/${encodeURIComponent(slug)}/attempts`, { rating }),
  exercises: (
    params: {
      stage_id?: string;
      difficulty?: Difficulty | '';
      status?: 'accepted' | 'unstarted' | '';
      cursor?: number;
      limit?: number;
    } = {},
    f = fetch
  ) => get<ExerciseListResponse>(f, `/exercises${queryString(params)}`),
  exercise: (slug: string, f = fetch) =>
    get<ExerciseDetailResponse>(f, `/exercises/${encodeURIComponent(slug)}`),
  exerciseDraft: (slug: string, language: Language, contract: ExerciseContract, f = fetch) =>
    get<ExerciseDraft | null>(
      f,
      `/exercises/${encodeURIComponent(slug)}/draft${queryString({ language, contract })}`
    ),
  putExerciseDraft: (
    slug: string,
    language: Language,
    contract: ExerciseContract,
    code: string,
    f = fetch
  ) =>
    send<ExerciseDraft>(f, 'PUT', `/exercises/${encodeURIComponent(slug)}/draft`, {
      language,
      contract,
      code
    }),
  createSubmission: (
    exercise_slug: string,
    language: Language,
    contract: ExerciseContract,
    source_code: string,
    f = fetch
  ) =>
    send<Submission>(f, 'POST', '/submissions', {
      exercise_slug,
      language,
      contract,
      source_code
    }),
  submission: (id: number, f = fetch) => get<Submission>(f, `/submissions/${id}`),
  submissions: (params: { cursor?: number; limit?: number; exercise_slug?: string } = {}, f = fetch) =>
    get<SubmissionListResponse>(f, `/submissions${queryString(params)}`),
  judgeStatus: (f = fetch) => get<JudgeStatusResponse>(f, '/judge/status'),

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
    send<{
      training: number;
      drafts: number;
      bookmarks: number;
      lessons: number;
      exercise_drafts: number;
      reviews: number;
    }>(f, 'POST', '/progress/import', body)
};
