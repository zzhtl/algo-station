import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  try {
    const data = await api.problems({ bookmarked: true, page_size: 200 }, fetch);
    return { items: data.items, loadError: false };
  } catch {
    return { items: [], loadError: true };
  }
};
