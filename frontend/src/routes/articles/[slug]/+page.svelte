<script lang="ts">
  import MarkdownView from '$lib/components/MarkdownView.svelte';
  import { ArrowLeft, ExternalLink } from 'lucide-svelte';

  export let data;
  $: article = data.article;
</script>

<div class="mx-auto max-w-5xl px-6 py-8">
  <a href="/articles" class="btn-ghost mb-4 !px-0 text-sm">
    <ArrowLeft size={14} /> 返回题解列表
  </a>

  <header class="mb-6">
    <div class="flex items-center gap-2 text-xs text-ink-dim">
      <span class="chip chip-tag">{article.category}</span>
    </div>
    <h1 class="mt-2 text-3xl font-bold tracking-tight text-ink">{article.title}</h1>
    <p class="mt-2 text-ink-mute">{article.summary}</p>
    {#if article.problem_ids.length > 0}
      <div class="mt-3 flex flex-wrap items-center gap-2 text-xs">
        <span class="text-ink-dim">涉及题目</span>
        {#each article.problem_ids as id}
          <a class="link inline-flex items-center gap-1 font-mono" href="/problems/{id}">
            #{id}
            <ExternalLink size={10} />
          </a>
        {/each}
      </div>
    {/if}
  </header>

  <article class="card p-6 sm:p-10">
    <MarkdownView source={article.content} />
  </article>
</div>
