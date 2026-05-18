import { error } from '@sveltejs/kit';
import { api } from '$lib/api';

export const load = async ({ fetch, params }) => {
  try {
    const article = await api.article(params.slug, fetch);
    return { article };
  } catch (e) {
    throw error(404, '题解不存在');
  }
};
