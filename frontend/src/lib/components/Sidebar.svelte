<script lang="ts">
  import { page } from '$app/stores';
  import {
    Home,
    ListChecks,
    BookOpen,
    Tags,
    Sparkles
  } from 'lucide-svelte';

  type NavItem = { href: string; label: string; icon: typeof Home; exact?: boolean };

  const nav: NavItem[] = [
    { href: '/', label: '首页', icon: Home, exact: true },
    { href: '/problems', label: '题库浏览', icon: ListChecks },
    { href: '/articles', label: '原创题解', icon: BookOpen },
    { href: '/tags', label: '算法分类', icon: Tags }
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
      <div class="text-[11px] text-ink-dim">算法学习站 · 原创题解</div>
    </div>
  </div>

  <nav class="flex-1 overflow-y-auto px-2 py-3">
    <div class="px-2 pb-1 text-[10px] font-semibold uppercase tracking-wider text-ink-dim">
      浏览
    </div>
    <ul class="space-y-0.5">
      {#each nav as item}
        {@const Active = isActive(item.href, item.exact)}
        <li>
          <a
            href={item.href}
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

  <div class="border-t border-bg-border px-4 py-3 text-[11px] text-ink-dim">
    <div>本地题库索引 · 原创讲解与练习</div>
  </div>
</aside>
