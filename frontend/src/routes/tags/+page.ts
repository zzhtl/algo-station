import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  try {
    const tags = await api.tags(fetch);
    return { tags, loadError: false };
  } catch {
    return { tags: [], loadError: true };
  }
};
