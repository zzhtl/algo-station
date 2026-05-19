<script lang="ts">
  import { Search, ArrowRight, BookOpen, ListChecks, Layers, Route, Zap } from 'lucide-svelte';

  export let data;
  $: stats = data.stats;
  $: articles = data.articles;

  const features = [
    {
      icon: Route,
      title: '训练路线',
      desc: '按“读懂、独立做、复盘”推进，把零散题解串成阶段目标。'
    },
    {
      icon: ListChecks,
      title: '完整题库浏览',
      desc: '3000+ 题目索引，按难度、标签、关键词筛选；进入详情后直接训练。'
    },
    {
      icon: BookOpen,
      title: '原创讲解',
      desc: '从思路到代码逐步拆解，配套 Mermaid 图示，多语言实现并行对照。'
    },
    {
      icon: Layers,
      title: '分类索引',
      desc: '按数据结构和算法范式组织内容：双指针、滑动窗口、回溯、DP、图论 ...'
    },
    {
      icon: Zap,
      title: '快速搜索',
      desc: '后端 SQLite 全文索引，毫秒级返回；前端预取，浏览顺滑无延迟。'
    }
  ];

  let searchQuery = '';
  function goSearch() {
    const q = searchQuery.trim();
    if (!q) return;
    location.href = `/problems?q=${encodeURIComponent(q)}`;
  }
</script>

<div class="mx-auto max-w-5xl px-6 py-12 lg:py-20">
  <section class="text-center">
    <div class="mb-3 inline-flex items-center gap-2 rounded-full border border-bg-border bg-bg-soft px-3 py-1 text-xs text-ink-mute">
      <Zap size={12} class="text-accent" />
      <span>Rust + SvelteKit · 原创算法学习站</span>
    </div>
    <h1 class="text-4xl font-bold tracking-tight text-ink sm:text-5xl">
      把算法<span class="text-accent">啃</span>下来
    </h1>
    <p class="mx-auto mt-4 max-w-2xl text-balance text-ink-mute">
      不是另一个题目索引。这里把题解、题库和训练路线串起来：
      先读懂模式，再独立写题，最后复盘可迁移的算法思维。
    </p>

    <div class="mx-auto mt-8 flex max-w-xl items-center gap-2">
      <div class="relative flex-1">
        <Search size={16} class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-ink-dim" />
        <input
          class="input pl-9"
          placeholder="按题号、英文/中文题名搜索 (例如：1 / two sum / 两数之和)"
          bind:value={searchQuery}
          on:keydown={(e) => e.key === 'Enter' && goSearch()}
        />
      </div>
      <button class="btn-primary" on:click={goSearch}>
        搜索 <ArrowRight size={14} />
      </button>
    </div>
    <div class="mt-4">
      <a href="/training" class="btn-ghost">
        进入训练路线 <ArrowRight size={14} />
      </a>
    </div>
  </section>

  {#if stats}
    <section class="mt-12 grid grid-cols-2 gap-3 sm:grid-cols-4">
      <div class="card p-4">
        <div class="text-xs text-ink-dim">题目总数</div>
        <div class="mt-1 text-2xl font-bold tabular-nums">{stats.total_problems.toLocaleString()}</div>
      </div>
      <div class="card p-4">
        <div class="text-xs text-ink-dim">简单 / 中等 / 困难</div>
        <div class="mt-1 flex items-baseline gap-2 text-lg font-semibold tabular-nums">
          <span class="text-easy">{stats.easy}</span>
          <span class="text-ink-dim">/</span>
          <span class="text-medium">{stats.medium}</span>
          <span class="text-ink-dim">/</span>
          <span class="text-hard">{stats.hard}</span>
        </div>
      </div>
      <div class="card p-4">
        <div class="text-xs text-ink-dim">标签数量</div>
        <div class="mt-1 text-2xl font-bold tabular-nums">{stats.total_tags}</div>
      </div>
      <div class="card p-4">
        <div class="text-xs text-ink-dim">原创题解</div>
        <div class="mt-1 text-2xl font-bold tabular-nums">{stats.total_articles}</div>
      </div>
    </section>
  {/if}

  <section class="mt-12 grid grid-cols-1 gap-4 sm:grid-cols-2">
    {#each features as f}
      <div class="card p-5">
        <div class="mb-3 inline-flex h-9 w-9 items-center justify-center rounded-lg bg-accent/10 text-accent ring-1 ring-accent/20">
          <svelte:component this={f.icon} size={18} />
        </div>
        <div class="text-base font-semibold text-ink">{f.title}</div>
        <p class="mt-1 text-sm text-ink-mute">{f.desc}</p>
      </div>
    {/each}
  </section>

  {#if articles.length > 0}
    <section class="mt-12">
      <div class="mb-3 flex items-baseline justify-between">
        <h2 class="text-lg font-semibold text-ink">原创题解 ↓</h2>
        <a class="link text-sm" href="/articles">查看全部 →</a>
      </div>
      <ul class="grid grid-cols-1 gap-2 sm:grid-cols-2">
        {#each articles.slice(0, 6) as a}
          <li>
            <a href="/articles/{a.slug}" class="card flex items-start gap-3 p-4 transition hover:border-accent/40 hover:bg-bg-card/80">
              <div class="grid h-8 w-8 shrink-0 place-items-center rounded-md bg-accent/10 text-accent ring-1 ring-accent/20">
                <BookOpen size={14} />
              </div>
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2">
                  <span class="chip chip-tag">{a.category}</span>
                  <span class="truncate font-medium text-ink">{a.title}</span>
                </div>
                <div class="mt-1 line-clamp-2 text-xs text-ink-mute">{a.summary}</div>
              </div>
            </a>
          </li>
        {/each}
      </ul>
    </section>
  {/if}

  <section class="mt-12 rounded-xl border border-bg-border bg-bg-soft/50 p-5 text-sm text-ink-mute">
    <div class="font-medium text-ink">关于版权</div>
    <p class="mt-1">
      本站用于个人算法训练和题解整理。题目资料来自本地索引，站内讲解、代码和图示按主题组织，方便反复练习。
    </p>
  </section>
</div>
