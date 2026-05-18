import { error } from '@sveltejs/kit';
import { api, type StatementResponse } from '$lib/api';

export const load = async ({ fetch, params }) => {
  try {
    const problem = await api.problem(params.id, fetch);
    let statementCn: StatementResponse | null = null;
    let statementEn: StatementResponse | null = null;
    if (problem.has_statement) {
      const [cn, en] = await Promise.all([
        api.statement(params.id, 'cn', fetch).catch(() => null),
        api.statement(params.id, 'en', fetch).catch(() => null)
      ]);
      statementCn = cn;
      statementEn = en;
    }
    return { problem, statementCn, statementEn };
  } catch {
    throw error(404, '题目不存在');
  }
};
