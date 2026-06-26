import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  try {
    const articles = await api.articles(fetch);
    return { articles, loadError: false };
  } catch {
    return { articles: [], loadError: true };
  }
};
