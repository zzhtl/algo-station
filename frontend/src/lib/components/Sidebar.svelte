<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { page } from '$app/stores';
  import { theme } from '$lib/stores/theme';
  import {
    Home,
    Brain,
    BookOpen,
    Code2,
    Route,
    Sparkles,
    Sun,
    Moon
  } from 'lucide-svelte';

  const dispatch = createEventDispatcher<{ navigate: void }>();

  type NavItem = { href: string; label: string; icon: typeof Home; exact?: boolean };

  const nav: NavItem[] = [
    { href: '/', label: '今日学习', icon: Home, exact: true },
    { href: '/roadmap', label: '学习路线', icon: Route },
    { href: '/practice', label: '配套训练', icon: Code2 },
    { href: '/reviews', label: '间隔复习', icon: Brain },
    { href: '/library', label: '算法资料库', icon: BookOpen }
  ];

  function isActive(href: string, exact?: boolean) {
    const path = $page.url.pathname;
    if (exact) return path === href;
    return path === href || path.startsWith(href + '/');
  }
</script>

<aside class="flex h-screen w-60 shrink-0 flex-col border-r border-bg-border bg-bg-soft/40">
  <div class="flex items-center gap-2 border-b border-bg-border px-4 py-4">
    <div class="grid h-8 w-8 place-items-center rounded-md bg-accent/15 text-accent ring-1 ring-accent/30">
      <Sparkles size={16} />
    </div>
    <div class="leading-tight">
      <div class="font-semibold tracking-tight text-ink">Algo Station</div>
      <div class="text-[11px] text-ink-dim">理解 · 训练 · 复习</div>
    </div>
  </div>

  <nav class="flex-1 overflow-y-auto px-2 py-3">
    <div class="px-2 pb-1 text-[10px] font-semibold uppercase tracking-wider text-ink-dim">
      学习
    </div>
    <ul class="space-y-0.5">
      {#each nav as item}
        {@const Active = isActive(item.href, item.exact)}
        <li>
          <a
            href={item.href}
            on:click={() => dispatch('navigate')}
            class="flex items-center gap-2 rounded-md px-3 py-2 text-sm transition
              {Active
                ? 'bg-accent/10 text-accent ring-1 ring-accent/20'
                : 'text-ink-mute hover:bg-bg-card hover:text-ink'}"
          >
            <svelte:component this={item.icon} size={16} />
            <span>{item.label}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <div class="border-t border-bg-border px-3 py-3">
    <button
      on:click={theme.toggle}
      class="flex w-full items-center gap-2 rounded-md px-3 py-2 text-sm text-ink-mute transition hover:bg-bg-card hover:text-ink"
    >
      {#if $theme === 'dark'}
        <Sun size={16} /> <span>浅色模式</span>
      {:else}
        <Moon size={16} /> <span>深色模式</span>
      {/if}
    </button>
    <div class="flex flex-wrap gap-x-3 gap-y-1 px-3 pt-2 text-[11px] text-ink-dim">
      <a href="/problems" class="hover:text-accent">完整题库</a>
      <a href="/articles" class="hover:text-accent">全部文章</a>
      <a href="/bookmarks" class="hover:text-accent">收藏</a>
    </div>
  </div>
</aside>
