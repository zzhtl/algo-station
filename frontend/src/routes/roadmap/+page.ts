import { api } from '$lib/api';

export const load = async ({ fetch }) => ({
  curriculum: await api.curriculum(fetch).catch(() => null)
});

