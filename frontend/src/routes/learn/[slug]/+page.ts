import { api } from '$lib/api';

export const load = async ({ params, fetch }) => {
  const lesson = await api.lesson(params.slug, fetch).catch(() => null);
  const article = lesson ? await api.article(lesson.lesson.article_slug, fetch).catch(() => null) : null;
  return { lesson, article };
};

