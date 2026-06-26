import { error } from '@sveltejs/kit';
import { api, type StatementResponse, type SolutionCode, type Draft } from '$lib/api';

export const load = async ({ fetch, params }) => {
  try {
    const problem = await api.problem(params.id, fetch);
    let statementCn: StatementResponse | null = null;
    let statementEn: StatementResponse | null = null;
    let solutions: SolutionCode[] = [];
    const drafts: Draft[] = await api.drafts(params.id, fetch).catch(() => []);
    if (problem.has_statement) {
      const [cn, en, sols] = await Promise.all([
        api.statement(params.id, 'cn', fetch).catch(() => null),
        api.statement(params.id, 'en', fetch).catch(() => null),
        api.solutions(params.id, 'go,rust,python', fetch).catch(() => null)
      ]);
      statementCn = cn;
      statementEn = en;
      solutions = sols?.solutions ?? [];
    }
    return { problem, statementCn, statementEn, solutions, drafts };
  } catch {
    throw error(404, '题目不存在');
  }
};
