import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { api } from '$lib/api';

function createBookmarks() {
  const { subscribe, set, update } = writable<Set<number>>(new Set());
  let loaded = false;

  async function load() {
    if (!browser || loaded) return;
    loaded = true;
    try {
      set(new Set(await api.bookmarks()));
    } catch {
      /* 后端不可用时保持空集 */
    }
  }

  async function toggle(id: number) {
    let adding = false;
    update((s) => {
      const next = new Set(s);
      adding = !next.has(id);
      adding ? next.add(id) : next.delete(id);
      return next;
    });
    try {
      adding ? await api.addBookmark(id) : await api.removeBookmark(id);
    } catch {
      // 写后端失败则回滚乐观更新
      update((s) => {
        const next = new Set(s);
        adding ? next.delete(id) : next.add(id);
        return next;
      });
    }
  }

  return { subscribe, load, toggle };
}

export const bookmarks = createBookmarks();
