<script lang="ts">
  import { ArrowRight, BookOpen, Layers, Library, Search } from 'lucide-svelte';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import type { ArticleListItem, Difficulty, Stats } from '$lib/api';

  export let data: { articles: ArticleListItem[]; stats: Stats | null };
  let query = '';
  let category = '';
  let difficulty: Difficulty | '' = '';
  $: categories = [...new Set(data.articles.map((article) => article.category))].sort();
  $: filtered = data.articles.filter((article) => {
    const needle = query.trim().toLowerCase();
    return (!needle || `${article.title} ${article.summary} ${article.problem_ids.join(' ')}`.toLowerCase().includes(needle))
      && (!category || article.category === category)
      && (!difficulty || article.difficulty === difficulty);
  });
</script>

<svelte:head><title>算法资料库 · Algo Station</title></svelte:head>

<div class="mx-auto max-w-6xl px-5 py-8 sm:px-7 lg:py-12">
  <header class="mb-7 flex flex-col justify-between gap-4 sm:flex-row sm:items-end"><div><div class="mb-2 flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-accent"><Library size={14} /> 资料库</div><h1 class="text-3xl font-bold tracking-tight text-ink">按需查阅，不打断学习主线</h1><p class="mt-2 max-w-2xl text-sm leading-6 text-ink-mute">保留全部 {data.articles.length} 篇算法文章与完整题库。课程负责循序渐进，资料库负责深入和查漏补缺。</p></div><div class="flex gap-2"><a class="btn-ghost" href="/problems">完整题库 <ArrowRight size={14} /></a><a class="btn-ghost" href="/tags">标签索引 <ArrowRight size={14} /></a></div></header>

  <section class="mb-5 grid gap-3 rounded-xl border border-bg-border bg-bg-card p-4 md:grid-cols-[minmax(15rem,1fr)_13rem_10rem]">
    <label class="relative"><Search size={15} class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-ink-dim" /><input class="input pl-9" placeholder="搜索标题、摘要或题号" bind:value={query} /></label>
    <select class="input" bind:value={category}><option value="">全部专题</option>{#each categories as item}<option value={item}>{item}</option>{/each}</select>
    <select class="input" bind:value={difficulty}><option value="">全部难度</option><option value="Easy">简单</option><option value="Medium">中等</option><option value="Hard">困难</option></select>
  </section>

  <div class="mb-3 flex items-center justify-between text-xs text-ink-dim"><span>{filtered.length} 篇结果</span>{#if data.stats}<span>{data.stats.total_problems.toLocaleString()} 道题 · {data.stats.total_tags} 个标签</span>{/if}</div>
  <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
    {#each filtered as article}
      <a href="/articles/{article.slug}" class="card group flex min-h-40 flex-col p-5 transition hover:border-accent/35 hover:-translate-y-0.5">
        <div class="flex items-start justify-between gap-3"><div class="grid h-9 w-9 place-items-center rounded-lg bg-accent/10 text-accent"><BookOpen size={17} /></div><DifficultyChip difficulty={article.difficulty} /></div>
        <h2 class="mt-4 line-clamp-2 font-semibold leading-6 text-ink group-hover:text-accent">{article.title}</h2>
        <p class="mt-2 line-clamp-2 text-xs leading-5 text-ink-mute">{article.summary}</p>
        <div class="mt-auto flex items-center justify-between pt-4 text-[11px] text-ink-dim"><span class="inline-flex items-center gap-1"><Layers size={12} /> {article.category}</span><span>{article.problem_ids.length} 道关联题</span></div>
      </a>
    {/each}
  </div>
  {#if !filtered.length}<div class="card p-10 text-center text-sm text-ink-mute">没有匹配的资料，试试放宽筛选条件。</div>{/if}
</div>

