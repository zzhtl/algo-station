import { api } from '$lib/api';

export const load = async ({ fetch, url }) => {
  const q = url.searchParams.get('q') ?? '';
  const difficulty = url.searchParams.get('difficulty') ?? '';
  const tag = url.searchParams.get('tag') ?? '';
  const has_article = url.searchParams.get('has_article') === '1';
  const page = Number(url.searchParams.get('page') ?? 1);

  const filters = { q, difficulty, tag, has_article, page };

  try {
    const [page_data, tags] = await Promise.all([
      api.problems(
        { q, difficulty, tag, has_article: has_article || undefined, page, page_size: 30 },
        fetch
      ),
      api.tags(fetch).catch(() => [])
    ]);
    return { pageData: page_data, tags, filters, loadError: false };
  } catch {
    return {
      pageData: { items: [], total: 0, page, page_size: 30 },
      tags: [],
      filters,
      loadError: true
    };
  }
};
