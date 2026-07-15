import { api } from '$lib/api';

export const load = async ({ fetch }) => ({ reviews: await api.reviews(fetch).catch(() => []) });

