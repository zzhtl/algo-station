import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  const [articles, stats] = await Promise.all([
    api.articles(fetch).catch(() => []),
    api.stats(fetch).catch(() => null)
  ]);
  return { articles, stats };
};

