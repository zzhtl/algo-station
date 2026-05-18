import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  const [stats, articles] = await Promise.all([
    api.stats(fetch).catch(() => null),
    api.articles(fetch).catch(() => [])
  ]);
  return { stats, articles };
};
