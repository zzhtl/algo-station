<script lang="ts">
  import { Star, FileText, Lock } from 'lucide-svelte';
  import type { ProblemListItem } from '$lib/api';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import BookmarkButton from '$lib/components/BookmarkButton.svelte';
  import ErrorState from '$lib/components/ErrorState.svelte';

  export let data;
  $: items = data.items as ProblemListItem[];
</script>

<div class="mx-auto max-w-5xl px-6 py-8">
  <div class="mb-6">
    <h1 class="flex items-center gap-2 text-2xl font-bold text-ink">
      <Star size={22} class="text-medium" /> 我的收藏
    </h1>
    <p class="mt-1 text-sm text-ink-mute">收藏的题目会保存在后端，换浏览器也不会丢失。</p>
  </div>

  {#if data.loadError}
    <ErrorState />
  {:else if items.length === 0}
    <div class="card p-10 text-center text-sm text-ink-mute">
      <Star size={24} class="mx-auto mb-2 text-ink-dim" />
      还没有收藏。在题库或题目详情页点 <Star size={12} class="inline" /> 即可收藏。
    </div>
  {:else}
    <ul class="space-y-2">
      {#each items as p (p.id)}
        <li class="card flex items-center gap-3 p-4 transition hover:border-accent/40">
          <span class="w-12 shrink-0 text-center font-mono text-sm text-ink-mute">{p.id}</span>
          <div class="min-w-0 flex-1">
            <a href="/problems/{p.id}" class="font-medium text-ink hover:text-accent">{p.title_cn}</a>
            <div class="mt-0.5 text-xs text-ink-dim">{p.title_en}</div>
            <div class="mt-1.5 flex flex-wrap items-center gap-1">
              <DifficultyChip difficulty={p.difficulty} />
              {#if p.is_premium}
                <span class="chip chip-tag !text-medium"><Lock size={10} /> 会员</span>
              {/if}
              {#if p.has_article}
                <span class="chip chip-tag !text-accent"><FileText size={10} /> 题解</span>
              {/if}
              {#each p.tags.slice(0, 4) as t}
                <a class="chip chip-tag hover:!text-accent" href="/problems?tag={t.slug}">{t.name_cn}</a>
              {/each}
            </div>
          </div>
          <BookmarkButton id={p.id} size={18} />
        </li>
      {/each}
    </ul>
  {/if}
</div>
