<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import ErrorState from '$lib/components/ErrorState.svelte';
  import BookmarkButton from '$lib/components/BookmarkButton.svelte';
  import { Search, FileText, Lock, ChevronLeft, ChevronRight, X } from 'lucide-svelte';

  export let data;
  $: pageData = data.pageData;
  $: tags = data.tags;
  $: filters = data.filters;

  let q = '';
  $: q = filters.q;

  function update(patch: Record<string, string | number | boolean | undefined>) {
    const url = new URL($page.url);
    for (const [k, v] of Object.entries(patch)) {
      if (v === undefined || v === '' || v === false) {
        url.searchParams.delete(k);
      } else {
        url.searchParams.set(k, String(v));
      }
    }
    if (!('page' in patch)) url.searchParams.delete('page');
    goto(url.pathname + url.search, { keepFocus: true });
  }

  function totalPages(total: number, size: number) {
    return Math.max(1, Math.ceil(total / size));
  }
</script>

<div class="mx-auto max-w-6xl px-6 py-8">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-ink">题库浏览</h1>
    <p class="mt-1 text-sm text-ink-mute">
      共 <span class="font-medium text-ink">{pageData.total.toLocaleString()}</span> 道题。
      数据来源：本地题库索引。
    </p>
  </div>

  <div class="card mb-4 grid grid-cols-1 gap-3 p-4 md:grid-cols-[1fr,auto,auto,auto]">
    <div class="relative">
      <Search size={16} class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-ink-dim" />
      <input
        class="input pl-9"
        placeholder="按题号 / 英文名 / 中文名 搜索"
        value={q}
        on:keydown={(e) => {
          if (e.key === 'Enter') update({ q: (e.target as HTMLInputElement).value });
        }}
      />
    </div>

    <select
      class="input min-w-[8rem]"
      value={filters.difficulty}
      on:change={(e) => update({ difficulty: (e.target as HTMLSelectElement).value })}
    >
      <option value="">全部难度</option>
      <option value="Easy">简单</option>
      <option value="Medium">中等</option>
      <option value="Hard">困难</option>
    </select>

    <select
      class="input min-w-[10rem]"
      value={filters.tag}
      on:change={(e) => update({ tag: (e.target as HTMLSelectElement).value })}
    >
      <option value="">全部标签</option>
      {#each tags as t}
        <option value={t.slug}>{t.name_cn} ({t.problem_count})</option>
      {/each}
    </select>

    <label class="inline-flex shrink-0 items-center gap-2 rounded-md border border-bg-border bg-bg-soft px-3 text-sm text-ink-mute">
      <input
        type="checkbox"
        class="accent-accent"
        checked={filters.has_article}
        on:change={(e) => update({ has_article: (e.target as HTMLInputElement).checked || undefined })}
      />
      仅看有题解
    </label>
  </div>

  {#if filters.q || filters.difficulty || filters.tag || filters.has_article}
    <div class="mb-3 flex items-center gap-2 text-xs text-ink-mute">
      <span>当前筛选：</span>
      {#if filters.q}
        <span class="chip chip-tag">关键词：{filters.q}</span>
      {/if}
      {#if filters.difficulty}
        <span class="chip chip-tag">难度：{filters.difficulty}</span>
      {/if}
      {#if filters.tag}
        <span class="chip chip-tag">标签：{filters.tag}</span>
      {/if}
      {#if filters.has_article}
        <span class="chip chip-tag">有题解</span>
      {/if}
      <button class="btn-ghost ml-1 !px-2 !py-0.5 text-xs" on:click={() => goto('/problems')}>
        <X size={12} /> 清空
      </button>
    </div>
  {/if}

  {#if data.loadError}
    <ErrorState />
  {:else}
  <div class="card overflow-hidden">
    <table class="w-full text-sm">
      <thead class="border-b border-bg-border bg-bg-soft/60 text-left text-xs text-ink-mute">
        <tr>
          <th class="w-16 px-4 py-2.5">#</th>
          <th class="px-4 py-2.5">题目</th>
          <th class="w-24 px-4 py-2.5">难度</th>
          <th class="w-32 px-4 py-2.5">通过率</th>
          <th class="w-32 px-4 py-2.5 text-right">操作</th>
        </tr>
      </thead>
      <tbody>
        {#each pageData.items as p (p.id)}
          <tr class="border-b border-bg-border/50 transition hover:bg-bg-soft/40">
            <td class="px-4 py-3 align-top tabular-nums text-ink-mute">{p.id}</td>
            <td class="px-4 py-3 align-top">
              <a href="/problems/{p.id}" class="font-medium text-ink hover:text-accent">
                {p.title_cn}
              </a>
              <div class="mt-0.5 text-xs text-ink-dim">{p.title_en}</div>
              <div class="mt-1.5 flex flex-wrap items-center gap-1">
                {#if p.is_premium}
                  <span class="chip chip-tag !text-medium" title="会员题"><Lock size={10} /> 会员</span>
                {/if}
                {#if p.has_article}
                  <span class="chip chip-tag !text-accent" title="有原创题解"><FileText size={10} /> 题解</span>
                {/if}
                {#each p.tags.slice(0, 4) as t}
                  <button
                    class="chip chip-tag hover:!text-accent"
                    on:click={() => update({ tag: t.slug })}
                  >
                    {t.name_cn}
                  </button>
                {/each}
                {#if p.tags.length > 4}
                  <span class="text-[10px] text-ink-dim">+{p.tags.length - 4}</span>
                {/if}
              </div>
            </td>
            <td class="px-4 py-3 align-top">
              <DifficultyChip difficulty={p.difficulty} />
            </td>
            <td class="px-4 py-3 align-top tabular-nums text-ink-mute">
              {p.acceptance_rate != null ? `${p.acceptance_rate.toFixed(1)}%` : '—'}
            </td>
            <td class="px-4 py-3 align-top">
              <div class="flex items-center justify-end gap-1">
                <BookmarkButton id={p.id} size={15} />
                <a href="/problems/{p.id}" class="btn-ghost !px-2" title="详情">
                  <FileText size={14} />
                </a>
              </div>
            </td>
          </tr>
        {/each}
        {#if pageData.items.length === 0}
          <tr><td colspan="5" class="px-4 py-16 text-center text-sm text-ink-dim">没有匹配的题目。</td></tr>
        {/if}
      </tbody>
    </table>
  </div>

  {#if pageData.total > pageData.page_size}
    {@const tp = totalPages(pageData.total, pageData.page_size)}
    <div class="mt-4 flex items-center justify-between text-sm text-ink-mute">
      <div>第 {pageData.page} / {tp} 页 · 共 {pageData.total.toLocaleString()} 条</div>
      <div class="flex items-center gap-2">
        <button
          class="btn-ghost"
          disabled={pageData.page <= 1}
          on:click={() => update({ page: pageData.page - 1 })}
        >
          <ChevronLeft size={14} /> 上一页
        </button>
        <button
          class="btn-ghost"
          disabled={pageData.page >= tp}
          on:click={() => update({ page: pageData.page + 1 })}
        >
          下一页 <ChevronRight size={14} />
        </button>
      </div>
    </div>
  {/if}
  {/if}
</div>
