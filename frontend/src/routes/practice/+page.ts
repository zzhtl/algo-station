import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  const [exercises, curriculum, judge] = await Promise.all([
    api.exercises({ limit: 60 }, fetch).catch(() => null),
    api.curriculum(fetch).catch(() => null),
    api.judgeStatus(fetch).catch(() => null)
  ]);
  return { exercises, curriculum, judge };
};

