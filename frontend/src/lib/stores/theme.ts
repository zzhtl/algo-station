import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark';

const KEY = 'algo-station:theme';

function createTheme() {
  // 初值与 app.html 的防闪烁脚本保持一致：读 <html> 上已经设好的 class。
  const initial: Theme = browser
    ? document.documentElement.classList.contains('dark')
      ? 'dark'
      : 'light'
    : 'dark';

  const { subscribe, set } = writable<Theme>(initial);

  function apply(t: Theme) {
    if (browser) {
      document.documentElement.classList.toggle('dark', t === 'dark');
      try {
        localStorage.setItem(KEY, t);
      } catch {
        /* localStorage 不可用时忽略 */
      }
      // 通知需要随主题重渲染的组件（如 Mermaid 图）。
      window.dispatchEvent(new CustomEvent('themechange', { detail: t }));
    }
    set(t);
  }

  return {
    subscribe,
    set: apply,
    toggle: () =>
      apply(browser && document.documentElement.classList.contains('dark') ? 'light' : 'dark')
  };
}

export const theme = createTheme();
