<script lang="ts">
  import { BookOpen } from 'lucide-svelte';
  import type { ArticleListItem } from '$lib/api';

  export let data;
  $: articles = data.articles;

  $: byCategory = articles.reduce<Record<string, ArticleListItem[]>>((acc, a) => {
    (acc[a.category] ??= []).push(a);
    return acc;
  }, {});
</script>

<div class="mx-auto max-w-5xl px-6 py-8">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-ink">原创题解</h1>
    <p class="mt-1 text-sm text-ink-mute">
      按算法范式和数据结构组织。所有内容原创，配套图示和多语言代码。
    </p>
  </div>

  {#if articles.length === 0}
    <div class="card p-10 text-center text-sm text-ink-mute">
      <BookOpen size={24} class="mx-auto mb-2 text-ink-dim" />
      还没有题解。运行 <code class="kbd">cargo run --bin import</code> 后请等等我补内容。
    </div>
  {:else}
    {#each Object.entries(byCategory) as [cat, items]}
      <section class="mb-8">
        <h2 class="mb-3 flex items-baseline gap-2 text-lg font-semibold text-ink">
          {cat}
          <span class="text-xs font-normal text-ink-dim">{items.length} 篇</span>
        </h2>
        <ul class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          {#each items as a}
            <li>
              <a href="/articles/{a.slug}" class="card flex h-full flex-col p-4 transition hover:border-accent/40 hover:bg-bg-card/80">
                <div class="font-medium text-ink">{a.title}</div>
                <div class="mt-1 line-clamp-3 flex-1 text-xs text-ink-mute">{a.summary}</div>
                {#if a.problem_ids.length > 0}
                  <div class="mt-3 flex flex-wrap gap-1 text-[10px] text-ink-dim">
                    {#each a.problem_ids as id}
                      <span class="font-mono">#{id}</span>
                    {/each}
                  </div>
                {/if}
              </a>
            </li>
          {/each}
        </ul>
      </section>
    {/each}
  {/if}
</div>
