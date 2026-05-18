import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  const articles = await api.articles(fetch).catch(() => []);
  return { articles };
};
