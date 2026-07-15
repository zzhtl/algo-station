import { api } from '$lib/api';

export const load = async ({ params, fetch }) => {
  const [exercise, judge, submissions] = await Promise.all([
    api.exercise(params.slug, fetch).catch(() => null),
    api.judgeStatus(fetch).catch(() => null),
    api.submissions({ exercise_slug: params.slug, limit: 8 }, fetch).catch(() => null)
  ]);
  const statement = exercise
    ? await api.statement(exercise.problem_id, 'cn', fetch).catch(() => null)
    : null;
  return { exercise, judge, submissions, statement };
};
