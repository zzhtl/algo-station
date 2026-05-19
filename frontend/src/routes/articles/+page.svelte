<script lang="ts">
  import { BookOpen } from 'lucide-svelte';
  import type { ArticleListItem, Difficulty } from '$lib/api';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';

  export let data;
  $: articles = data.articles;

  const difficultyOrder: Difficulty[] = ['Easy', 'Medium', 'Hard'];
  const difficultyTitle: Record<Difficulty, string> = {
    Easy: '简单',
    Medium: '中等',
    Hard: '困难'
  };

  function articleDifficulty(article: ArticleListItem): Difficulty {
    return difficultyOrder.includes(article.difficulty) ? article.difficulty : 'Medium';
  }

  $: byDifficulty = difficultyOrder.map((difficulty) => ({
    difficulty,
    items: articles
      .filter((a: ArticleListItem) => articleDifficulty(a) === difficulty)
      .sort((a: ArticleListItem, b: ArticleListItem) =>
        a.category.localeCompare(b.category) || a.order_in_cat - b.order_in_cat || a.title.localeCompare(b.title)
      )
  }));

  function groupedByCategory(items: ArticleListItem[]) {
    return items.reduce<Record<string, ArticleListItem[]>>((acc, a) => {
      (acc[a.category] ??= []).push(a);
      return acc;
    }, {});
  }
</script>

<div class="mx-auto max-w-5xl px-6 py-8">
  <div class="mb-6">
    <h1 class="text-2xl font-bold text-ink">原创题解</h1>
    <p class="mt-1 text-sm text-ink-mute">
      按简单、中等、困难由易到难组织，再按算法范式细分。难度由关联题目的最高难度推导。
    </p>
  </div>

  {#if articles.length === 0}
    <div class="card p-10 text-center text-sm text-ink-mute">
      <BookOpen size={24} class="mx-auto mb-2 text-ink-dim" />
      还没有题解。运行 <code class="kbd">cargo run --bin import</code> 后请等等我补内容。
    </div>
  {:else}
    {#each byDifficulty as section}
      {#if section.items.length > 0}
        <section class="mb-10">
          <h2 class="mb-4 flex items-center gap-2 text-lg font-semibold text-ink">
            {difficultyTitle[section.difficulty]}
            <DifficultyChip difficulty={section.difficulty} />
            <span class="text-xs font-normal text-ink-dim">{section.items.length} 篇</span>
          </h2>

          {#each Object.entries(groupedByCategory(section.items)) as [cat, items]}
            <section class="mb-6">
              <h3 class="mb-3 flex items-baseline gap-2 text-base font-semibold text-ink">
                {cat}
                <span class="text-xs font-normal text-ink-dim">{items.length} 篇</span>
              </h3>
              <ul class="grid grid-cols-1 gap-2 sm:grid-cols-2">
                {#each items as a}
                  <li>
                    <a href="/articles/{a.slug}" class="card flex h-full flex-col p-4 transition hover:border-accent/40 hover:bg-bg-card/80">
                      <div class="flex flex-wrap items-center gap-2">
                        <DifficultyChip difficulty={articleDifficulty(a)} />
                        <span class="font-medium text-ink">{a.title}</span>
                      </div>
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
        </section>
      {/if}
    {/each}
  {/if}
</div>
