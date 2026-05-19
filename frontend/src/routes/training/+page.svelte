<script lang="ts">
  import { onMount } from 'svelte';
  import {
    BookOpen,
    CheckCircle2,
    ChevronDown,
    Flame,
    ListChecks,
    NotebookPen,
    RotateCcw,
    SquarePen,
    Target
  } from 'lucide-svelte';
  import type { ArticleListItem, Difficulty } from '$lib/api';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';

  export let data;
  $: articles = data.articles as ArticleListItem[];
  $: stats = data.stats;

  type ArticleStatus = 'todo' | 'learned' | 'practiced' | 'reviewed';
  type StatusFilter = 'all' | ArticleStatus;
  type AttemptResult = '' | '独立完成' | '提示后完成' | '未完成';

  type TrainingRecord = {
    articleKey?: string;
    articleSlug?: string;
    status?: ArticleStatus;
    patternNote?: string;
    completedProblems?: number[];
    attemptResult?: AttemptResult;
    stuckNote?: string;
    reviewNote?: string;
    updatedAt?: string;
  };

  type TrainingItem = {
    article: ArticleListItem;
    articleKey: string;
    record: TrainingRecord;
    status: ArticleStatus;
  };

  const storageKey = 'algo-station:training-records:v3';
  const legacyStorageKey = 'algo-station:training-records:v2';
  const statusMeta: Record<ArticleStatus, { label: string; className: string }> = {
    todo: { label: '未开始', className: 'text-ink-dim' },
    learned: { label: '读懂', className: 'text-medium' },
    practiced: { label: '独立做', className: 'text-accent' },
    reviewed: { label: '已复盘', className: 'text-easy' }
  };
  const difficultyOrder: Difficulty[] = ['Easy', 'Medium', 'Hard'];

  const phases = [
    {
      name: '第一阶段',
      title: '线性结构与枚举边界',
      categories: ['数组与字符串', '哈希与前缀', '链表', '栈与队列'],
      focus: '先把指针、窗口、前缀、栈队列这些基础控制流练稳。',
      checkpoints: ['能说清指针为什么移动', '能识别重复计算', '能写出边界样例']
    },
    {
      name: '第二阶段',
      title: '搜索、二分与结构化递归',
      categories: ['二分搜索', '二叉树', '回溯', '搜索与图论'],
      focus: '训练“状态空间”意识：搜索什么、剪掉什么、什么时候收敛。',
      checkpoints: ['能定义搜索状态', '能证明二分单调性', '能控制递归出口']
    },
    {
      name: '第三阶段',
      title: '最优子结构与策略选择',
      categories: ['动态规划', '贪心', '堆与优先队列'],
      focus: '把题目从“怎么模拟”提升到“状态、选择、局部最优是否成立”。',
      checkpoints: ['能写出状态含义', '能解释转移来源', '能判断贪心交换条件']
    },
    {
      name: '第四阶段',
      title: '高级结构与综合题',
      categories: ['并查集', '设计与数据结构', '数学与位运算'],
      focus: '把数据结构当成约束工具，处理连通性、缓存、计数、位状态和系统化设计。',
      checkpoints: ['能选择合适结构', '能分析操作复杂度', '能复盘同题多解差异']
    }
  ];

  const reviewPrompts = [
    '这题的“状态”或“不变量”是什么？',
    '哪一步减少了候选空间，为什么不会漏解？',
    '如果输入为空、重复、极大，会踩哪个边界？',
    '同类题下次出现时，第一反应应该是什么？'
  ];

  let recordsByKey: Record<string, TrainingRecord> = {};
  let filter: StatusFilter = 'all';
  let activeSlug = '';

  onMount(() => {
    recordsByKey = loadRecords();
  });

  $: plannedArticles = phases.flatMap((phase) => phaseArticles(phase));
  $: trainingItems = plannedArticles.map((article) => toTrainingItem(article));
  $: phaseViews = phases.map((phase) => {
    const allItems = trainingItems.filter((item) => phase.categories.includes(item.article.category));
    const items = filter === 'all' ? allItems : allItems.filter((item) => item.status === filter);
    return {
      ...phase,
      allItems,
      items,
      done: allItems.filter((item) => item.status === 'reviewed').length
    };
  });
  $: completedCount = trainingItems.filter((item) => item.status === 'reviewed').length;
  $: activeCount = trainingItems.filter((item) => {
    const status = item.status;
    return status === 'learned' || status === 'practiced';
  }).length;
  $: untouchedCount = trainingItems.filter((item) => item.status === 'todo').length;
  $: completionRate = trainingItems.length === 0 ? 0 : Math.round((completedCount / trainingItems.length) * 100);

  function phaseArticles(phase: { categories: string[] }) {
    return articles
      .filter((article) => phase.categories.includes(article.category))
      .sort((a, b) =>
        phase.categories.indexOf(a.category) - phase.categories.indexOf(b.category) ||
        difficultyRank(articleDifficulty(a)) - difficultyRank(articleDifficulty(b)) ||
        a.order_in_cat - b.order_in_cat ||
        a.title.localeCompare(b.title)
      );
  }

  function articleDifficulty(article: ArticleListItem): Difficulty {
    return difficultyOrder.includes(article.difficulty) ? article.difficulty : 'Medium';
  }

  function difficultyRank(difficulty: Difficulty) {
    return difficultyOrder.indexOf(difficulty);
  }

  function toTrainingItem(article: ArticleListItem): TrainingItem {
    const articleKey = keyFor(article.slug);
    const record = recordFor(article.slug);
    return {
      article,
      articleKey,
      record,
      status: record.status ?? deriveStatus(record)
    };
  }

  function recordFor(slug: string): TrainingRecord {
    return recordsByKey[keyFor(slug)] ?? {};
  }

  function statusFor(slug: string): ArticleStatus {
    return recordFor(slug).status ?? deriveStatus(recordFor(slug));
  }

  function deriveStatus(record: TrainingRecord): ArticleStatus {
    if ((record.reviewNote ?? '').trim()) return 'reviewed';
    if ((record.completedProblems ?? []).length > 0 && record.attemptResult) return 'practiced';
    if ((record.patternNote ?? '').trim()) return 'learned';
    return 'todo';
  }

  function nextAction(slug: string) {
    const status = statusFor(slug);
    if (status === 'todo') return '先读文章并写下模式总结';
    if (status === 'learned') return '进入相关题目独立练习';
    if (status === 'practiced') return '写复盘，总结迁移条件';
    return '已完成，后续可筛选复习';
  }

  function articleHref(slug: string) {
    return `/articles/${slug}`;
  }

  function problemHref(id: number) {
    return `/problems/${id}`;
  }

  function updateRecord(slug: string, patch: TrainingRecord) {
    const nextRecord = normalizeRecord(slug, { ...recordFor(slug), ...patch });
    recordsByKey = {
      ...recordsByKey,
      [keyFor(slug)]: nextRecord
    };
    persist();
  }

  function setProblemDone(slug: string, id: number, done: boolean) {
    const current = recordFor(slug).completedProblems ?? [];
    const next = done ? Array.from(new Set([...current, id])) : current.filter((item) => item !== id);
    updateRecord(slug, { completedProblems: next });
  }

  function resetAll() {
    recordsByKey = {};
    activeSlug = '';
    persist();
  }

  function resetArticle(slug: string) {
    const next = { ...recordsByKey };
    delete next[keyFor(slug)];
    recordsByKey = next;
    persist();
  }

  function keyFor(slug: string) {
    return `article:${slug}`;
  }

  function normalizeRecord(slug: string, record: TrainingRecord): TrainingRecord {
    const next = {
      ...record,
      articleKey: keyFor(slug),
      articleSlug: slug
    };
    return {
      ...next,
      status: deriveStatus(next),
      updatedAt: new Date().toISOString()
    };
  }

  function loadRecords(): Record<string, TrainingRecord> {
    try {
      const raw = localStorage.getItem(storageKey);
      if (raw) return JSON.parse(raw);

      const legacyRaw = localStorage.getItem(legacyStorageKey);
      if (!legacyRaw) return {};

      const legacy = JSON.parse(legacyRaw) as Record<string, TrainingRecord>;
      const migrated: Record<string, TrainingRecord> = {};
      for (const [slug, record] of Object.entries(legacy)) {
        migrated[keyFor(slug)] = normalizeRecord(slug, record);
      }
      localStorage.setItem(storageKey, JSON.stringify(migrated));
      return migrated;
    } catch {
      return {};
    }
  }

  function persist() {
    if (typeof localStorage === 'undefined') return;
    localStorage.setItem(storageKey, JSON.stringify(recordsByKey));
  }
</script>

<div class="mx-auto max-w-6xl px-6 py-8">
  <header class="mb-6">
    <div class="mb-2 flex items-center gap-2 text-xs text-ink-dim">
      <Target size={14} class="text-accent" />
      <span>算法训练路线</span>
    </div>
    <h1 class="text-2xl font-bold text-ink">从题目到算法思维</h1>
    <p class="mt-1 max-w-3xl text-sm text-ink-mute">
      训练状态不再靠点一个按钮确认，而是由你的阅读总结、完成题目和复盘记录自动推导。
    </p>
  </header>

  <section class="mb-6 grid grid-cols-2 gap-3 lg:grid-cols-5">
    <div class="card p-4">
      <div class="text-xs text-ink-dim">路线进度</div>
      <div class="mt-1 text-2xl font-bold tabular-nums text-ink">{completionRate}%</div>
    </div>
    <div class="card p-4">
      <div class="text-xs text-ink-dim">已复盘</div>
      <div class="mt-1 text-2xl font-bold tabular-nums text-easy">{completedCount}</div>
    </div>
    <div class="card p-4">
      <div class="text-xs text-ink-dim">进行中</div>
      <div class="mt-1 text-2xl font-bold tabular-nums text-accent">{activeCount}</div>
    </div>
    <div class="card p-4">
      <div class="text-xs text-ink-dim">未开始</div>
      <div class="mt-1 text-2xl font-bold tabular-nums text-ink">{untouchedCount}</div>
    </div>
    <div class="card p-4">
      <div class="text-xs text-ink-dim">题库规模</div>
      <div class="mt-1 text-2xl font-bold tabular-nums text-ink">
        {stats ? stats.total_problems.toLocaleString() : '-'}
      </div>
    </div>
  </section>

  <section class="mb-6 grid gap-4 lg:grid-cols-[minmax(0,1fr),20rem]">
    <div class="card p-5">
      <section class="mb-5 rounded-lg border border-bg-border bg-bg-soft/30 p-4">
        <h2 class="flex items-center gap-2 text-lg font-semibold text-ink">
          <Target size={17} /> 怎么训练
        </h2>
        <div class="mt-4 grid gap-3 md:grid-cols-4">
          <div class="rounded-lg border border-bg-border bg-bg-card p-3">
            <div class="text-xs font-medium text-accent">1. 读框架</div>
            <p class="mt-1 text-xs text-ink-mute">点文章标题阅读，回来写一句自己的模式总结。</p>
          </div>
          <div class="rounded-lg border border-bg-border bg-bg-card p-3">
            <div class="text-xs font-medium text-accent">2. 做题</div>
            <p class="mt-1 text-xs text-ink-mute">点相关题目进入详情页，在练习区手写，完成后勾选题号。</p>
          </div>
          <div class="rounded-lg border border-bg-border bg-bg-card p-3">
            <div class="text-xs font-medium text-accent">3. 记录结果</div>
            <p class="mt-1 text-xs text-ink-mute">记录是独立完成、提示后完成，还是没做出来。</p>
          </div>
          <div class="rounded-lg border border-bg-border bg-bg-card p-3">
            <div class="text-xs font-medium text-accent">4. 复盘迁移</div>
            <p class="mt-1 text-xs text-ink-mute">写下下次识别这类题的信号，系统才标为已复盘。</p>
          </div>
        </div>
      </section>

      <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
        <div>
          <h2 class="flex items-center gap-2 text-lg font-semibold text-ink">
            <ListChecks size={17} /> 训练推进
          </h2>
          <p class="mt-1 text-sm text-ink-mute">展开一篇内容，按流程完成。每个分类按简单、中等、困难推进。</p>
        </div>
        <div class="flex flex-wrap items-center gap-2">
          {#each ['all', 'todo', 'learned', 'practiced', 'reviewed'] as item}
            <button
              class="rounded-md border px-3 py-1.5 text-xs transition
                {filter === item
                  ? 'border-accent/40 bg-accent/15 text-accent'
                  : 'border-bg-border bg-bg-soft text-ink-mute hover:text-ink'}"
              on:click={() => (filter = item as StatusFilter)}
            >
              {item === 'all' ? '全部' : statusMeta[item as ArticleStatus].label}
            </button>
          {/each}
          <button class="btn-ghost text-xs" on:click={resetAll}>
            <RotateCcw size={13} /> 清空
          </button>
        </div>
      </div>

      <div class="space-y-5">
        {#each phaseViews as phase}
          <section class="rounded-lg border border-bg-border bg-bg-soft/30">
            <div class="border-b border-bg-border px-4 py-3">
              <div class="flex flex-wrap items-baseline justify-between gap-2">
                <div>
                  <div class="text-xs font-medium text-accent">{phase.name}</div>
                  <h3 class="mt-0.5 text-base font-semibold text-ink">{phase.title}</h3>
                </div>
                <div class="text-xs tabular-nums text-ink-dim">{phase.done} / {phase.allItems.length} 已复盘</div>
              </div>
              <p class="mt-2 text-sm text-ink-mute">{phase.focus}</p>
              <div class="mt-3 flex flex-wrap gap-1.5">
                {#each phase.checkpoints as checkpoint}
                  <span class="chip chip-tag">{checkpoint}</span>
                {/each}
              </div>
            </div>

            {#if phase.items.length === 0}
              <div class="px-4 py-8 text-center text-sm text-ink-dim">当前筛选下没有内容。</div>
            {:else}
              <ul class="divide-y divide-bg-border/70">
                {#each phase.items as item (item.articleKey)}
                  {@const article = item.article}
                  {@const record = item.record}
                  {@const status = item.status}
                  {@const open = activeSlug === article.slug}
                  <li class="px-4 py-3">
                    <div class="grid gap-3 sm:grid-cols-[minmax(0,1fr),auto] sm:items-start">
                      <div class="min-w-0">
                        <div class="flex flex-wrap items-center gap-2">
                          <span class="chip chip-tag">{article.category}</span>
                          <DifficultyChip difficulty={articleDifficulty(article)} />
                          <span class="chip chip-tag {statusMeta[status].className}">{statusMeta[status].label}</span>
                          <a href={articleHref(article.slug)} class="font-medium text-ink hover:text-accent">{article.title}</a>
                        </div>
                        <p class="mt-1 line-clamp-2 text-xs text-ink-mute">{article.summary}</p>
                        <div class="mt-2 text-xs text-ink-dim">下一步：{nextAction(article.slug)}</div>
                      </div>
                      <button
                        class="btn-ghost justify-self-start text-xs sm:justify-self-end"
                        on:click={() => (activeSlug = open ? '' : article.slug)}
                      >
                        {open ? '收起流程' : status === 'todo' ? '开始训练' : '继续训练'}
                        <ChevronDown size={13} class={open ? 'rotate-180' : ''} />
                      </button>
                    </div>

                    {#if open}
                      <div class="mt-4 grid gap-3 rounded-lg border border-bg-border bg-bg-card p-4">
                        <section class="rounded-lg border border-bg-border bg-bg-soft/30 p-3">
                          <div class="flex flex-wrap items-center justify-between gap-2">
                            <h4 class="font-medium text-ink">1. 阅读并提炼模式</h4>
                            <a class="link text-xs" href={articleHref(article.slug)}>打开文章</a>
                          </div>
                          <textarea
                            class="input mt-3 min-h-20 resize-y"
                            value={record.patternNote ?? ''}
                            on:input={(e) => updateRecord(article.slug, { patternNote: (e.target as HTMLTextAreaElement).value })}
                            placeholder="用你自己的话写：这类题的核心模式、不变量或状态定义是什么？"
                          ></textarea>
                        </section>

                        <section class="rounded-lg border border-bg-border bg-bg-soft/30 p-3">
                          <h4 class="font-medium text-ink">2. 进入相关题目并完成练习</h4>
                          {#if article.problem_ids.length > 0}
                            <div class="mt-3 grid gap-2 sm:grid-cols-2">
                              {#each article.problem_ids as id}
                                {@const checked = (record.completedProblems ?? []).includes(id)}
                                <div class="flex items-center gap-2 rounded-md border border-bg-border bg-bg-card p-2 text-xs text-ink-mute">
                                  <input
                                    type="checkbox"
                                    class="accent-accent"
                                    checked={checked}
                                    on:change={(e) => setProblemDone(article.slug, id, (e.target as HTMLInputElement).checked)}
                                  />
                                  <a class="link inline-flex items-center gap-1 font-mono" href={problemHref(id)}>
                                    <SquarePen size={12} /> #{id}
                                  </a>
                                </div>
                              {/each}
                            </div>
                          {:else}
                            <div class="mt-3 text-sm text-ink-dim">这篇文章还没有绑定题目。</div>
                          {/if}
                        </section>

                        <section class="rounded-lg border border-bg-border bg-bg-soft/30 p-3">
                          <h4 class="font-medium text-ink">3. 记录做题结果</h4>
                          <div class="mt-3 flex flex-wrap gap-2">
                            {#each ['独立完成', '提示后完成', '未完成'] as result}
                              <button
                                class="rounded-md border px-3 py-1.5 text-xs transition
                                  {record.attemptResult === result
                                    ? 'border-accent/40 bg-accent/15 text-accent'
                                    : 'border-bg-border bg-bg-card text-ink-mute hover:text-ink'}"
                                on:click={() => updateRecord(article.slug, { attemptResult: result as AttemptResult })}
                              >
                                {result}
                              </button>
                            {/each}
                          </div>
                          <textarea
                            class="input mt-3 min-h-16 resize-y"
                            value={record.stuckNote ?? ''}
                            on:input={(e) => updateRecord(article.slug, { stuckNote: (e.target as HTMLTextAreaElement).value })}
                            placeholder="卡在哪里？边界、状态定义、复杂度，还是代码实现？"
                          ></textarea>
                        </section>

                        <section class="rounded-lg border border-bg-border bg-bg-soft/30 p-3">
                          <h4 class="font-medium text-ink">4. 复盘迁移</h4>
                          <p class="mt-1 text-xs text-ink-dim">回答：下次看到什么信号，就应该想到这个模式？</p>
                          <textarea
                            class="input mt-3 min-h-20 resize-y"
                            value={record.reviewNote ?? ''}
                            on:input={(e) => updateRecord(article.slug, { reviewNote: (e.target as HTMLTextAreaElement).value })}
                            placeholder="写出识别信号、关键不变量、易错边界。写完后状态自动变成“已复盘”。"
                          ></textarea>
                        </section>

                        <div class="flex flex-wrap items-center justify-between gap-2 text-xs text-ink-dim">
                          <span>当前状态：<span class={statusMeta[status].className}>{statusMeta[status].label}</span></span>
                          <button class="btn-ghost text-xs" on:click={() => resetArticle(article.slug)}>
                            <RotateCcw size={13} /> 重置这篇
                          </button>
                        </div>
                      </div>
                    {/if}
                  </li>
                {/each}
              </ul>
            {/if}
          </section>
        {/each}
      </div>
    </div>

    <aside class="space-y-4">
      <section class="card p-5">
        <h2 class="flex items-center gap-2 text-lg font-semibold text-ink">
          <Flame size={17} /> 每日节奏
        </h2>
        <div class="mt-4 space-y-3 text-sm">
          <div class="rounded-lg border border-bg-border bg-bg-soft/40 p-3">
            <div class="font-medium text-ink">20 分钟读框架</div>
            <p class="mt-1 text-xs text-ink-mute">只抓状态、不变量、转移和边界，不急着背代码。</p>
          </div>
          <div class="rounded-lg border border-bg-border bg-bg-soft/40 p-3">
            <div class="font-medium text-ink">40 分钟独立写题</div>
            <p class="mt-1 text-xs text-ink-mute">卡住时先写失败原因，再打开题解对照。</p>
          </div>
          <div class="rounded-lg border border-bg-border bg-bg-soft/40 p-3">
            <div class="font-medium text-ink">10 分钟复盘迁移</div>
            <p class="mt-1 text-xs text-ink-mute">把这题归到一个模式，而不是归到一个答案。</p>
          </div>
        </div>
      </section>

      <section class="card p-5">
        <h2 class="flex items-center gap-2 text-lg font-semibold text-ink">
          <NotebookPen size={17} /> 复盘问题
        </h2>
        <ul class="mt-4 space-y-2 text-sm text-ink-mute">
          {#each reviewPrompts as prompt}
            <li class="flex gap-2">
              <BookOpen size={14} class="mt-0.5 shrink-0 text-accent" />
              <span>{prompt}</span>
            </li>
          {/each}
        </ul>
      </section>
    </aside>
  </section>
</div>
