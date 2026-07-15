import { api } from '$lib/api';

export const load = async ({ fetch }) => {
  const dashboard = await api.dashboard(fetch).catch(() => null);
  return { dashboard };
};

