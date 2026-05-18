import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  const tags = await api.tags(fetch).catch(() => []);
  return { tags };
};
