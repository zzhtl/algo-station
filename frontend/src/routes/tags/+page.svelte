<script lang="ts">
  import ErrorState from '$lib/components/ErrorState.svelte';
  export let data;
  $: tags = data.tags;
</script>

<div class="mx-auto max-w-5xl px-6 py-8">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-ink">算法分类</h1>
    <p class="mt-1 text-sm text-ink-mute">
      共 {tags.length} 个算法/数据结构标签。点击进入该分类下的题目列表，开始针对性训练。
    </p>
  </div>

  {#if data.loadError}
    <ErrorState />
  {:else}
  <div class="grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-4">
    {#each tags as t}
      <a href="/problems?tag={t.slug}" class="card flex items-center justify-between p-3 transition hover:border-accent/40">
        <div class="min-w-0">
          <div class="truncate font-medium text-ink">{t.name_cn}</div>
          <div class="truncate text-[11px] text-ink-dim">{t.name_en}</div>
        </div>
        <span class="ml-2 shrink-0 rounded-md bg-bg-soft px-2 py-0.5 text-xs tabular-nums text-ink-mute">{t.problem_count}</span>
      </a>
    {/each}
  </div>
  {/if}
</div>
