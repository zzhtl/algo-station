<script lang="ts">
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import MarkdownView from '$lib/components/MarkdownView.svelte';
  import { ArrowLeft, BookOpen, Lock, FileText, Eye, EyeOff, Code2, RotateCcw } from 'lucide-svelte';

  export let data;
  $: p = data.problem;
  $: stmtCn = data.statementCn?.content ?? '';
  $: stmtEn = data.statementEn?.content ?? '';
  $: hasCn = stmtCn.trim().length > 0;
  $: hasEn = stmtEn.trim().length > 0;

  let stmtLang: 'cn' | 'en' = 'cn';
  let showSolution = false;
  let problemMarkdown = '';
  let solutionMarkdown = '';
  let practiceLang = 'go';
  let practiceCode = '';
  let loadedPracticeKey = '';

  $: {
    if (stmtLang === 'cn' && !hasCn && hasEn) stmtLang = 'en';
    if (stmtLang === 'en' && !hasEn && hasCn) stmtLang = 'cn';
  }
  $: ({ problemMarkdown, solutionMarkdown } = splitStatement(stmtLang === 'cn' ? stmtCn : stmtEn));
  $: loadPractice(p.id, practiceLang);

  function splitStatement(source: string) {
    const match = /\n##\s+(解法|Solution)(?=\s|$)/.exec(source);
    if (!match) return { problemMarkdown: source, solutionMarkdown: '' };
    return {
      problemMarkdown: source.slice(0, match.index).trim(),
      solutionMarkdown: source.slice(match.index).trim()
    };
  }

  function loadPractice(problemId: number, lang: string) {
    const key = `algo-station:practice:${problemId}:${lang}`;
    if (key === loadedPracticeKey) return;
    loadedPracticeKey = key;
    if (typeof localStorage === 'undefined') {
      practiceCode = defaultPracticeCode(lang);
      return;
    }
    practiceCode = localStorage.getItem(key) ?? defaultPracticeCode(lang);
  }

  function savePractice() {
    if (typeof localStorage === 'undefined') return;
    localStorage.setItem(loadedPracticeKey, practiceCode);
  }

  function resetPractice() {
    practiceCode = defaultPracticeCode(practiceLang);
    savePractice();
  }

  function defaultPracticeCode(lang: string): string {
    if (lang === 'rust') {
      return `impl Solution {\n    pub fn solve() {\n        \n    }\n}`;
    }
    if (lang === 'python') {
      return `class Solution:\n    def solve(self):\n        pass`;
    }
    return `func solve() {\n\t\n}`;
  }
</script>

<div class="mx-auto max-w-4xl px-6 py-8">
  <a href="/problems" class="btn-ghost mb-4 !px-0 text-sm">
    <ArrowLeft size={14} /> 返回题库
  </a>

  <div class="card p-6">
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0">
        <div class="flex items-center gap-2 text-xs text-ink-dim">
          <span class="font-mono">#{p.id}</span>
          <DifficultyChip difficulty={p.difficulty} />
          {#if p.is_premium}
            <span class="chip chip-tag !text-medium"><Lock size={10} /> 会员</span>
          {/if}
          {#if p.acceptance_rate != null}
            <span class="text-ink-mute">通过率 {p.acceptance_rate.toFixed(1)}%</span>
          {/if}
        </div>
        <h1 class="mt-2 text-2xl font-bold text-ink">{p.title_cn}</h1>
        <div class="mt-1 text-sm text-ink-mute">{p.title_en}</div>
      </div>
    </div>

    {#if p.tags.length > 0}
      <div class="mt-4 flex flex-wrap items-center gap-1.5">
        <span class="text-xs text-ink-dim">标签</span>
        {#each p.tags as t}
          <a class="chip chip-tag hover:!text-accent" href="/problems?tag={t.slug}">
            {t.name_cn}
          </a>
        {/each}
      </div>
    {/if}
  </div>

  {#if hasCn || hasEn}
    <section class="mt-6">
      <div class="mb-3 flex items-center justify-between">
        <h2 class="flex items-center gap-2 text-lg font-semibold text-ink">
          <FileText size={16} /> 题目
        </h2>
        <div class="flex items-center gap-2">
          {#if solutionMarkdown}
            <button class="btn-ghost text-xs" on:click={() => (showSolution = !showSolution)}>
              {#if showSolution}
                <EyeOff size={14} /> 隐藏题解
              {:else}
                <Eye size={14} /> 显示题解
              {/if}
            </button>
          {/if}
          {#if hasCn && hasEn}
            <div class="flex overflow-hidden rounded-md border border-bg-border text-xs">
              <button
                class="px-3 py-1 transition {stmtLang === 'cn' ? 'bg-accent/20 text-accent' : 'text-ink-mute hover:text-ink'}"
                on:click={() => (stmtLang = 'cn')}
              >中文</button>
              <button
                class="px-3 py-1 transition {stmtLang === 'en' ? 'bg-accent/20 text-accent' : 'text-ink-mute hover:text-ink'}"
                on:click={() => (stmtLang = 'en')}
              >English</button>
            </div>
          {/if}
        </div>
      </div>
      <div class="card p-6">
        <MarkdownView source={problemMarkdown} />
        {#if solutionMarkdown}
          <div class="mt-6 border-t border-bg-border pt-6">
            {#if showSolution}
              <MarkdownView source={solutionMarkdown} />
            {:else}
              <div class="rounded-md border border-bg-border bg-bg-soft/50 p-4 text-sm text-ink-mute">
                题解已隐藏，先在下面的练习区自己写一遍。
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </section>
  {/if}

  <section class="mt-6">
    <div class="mb-3 flex items-center justify-between">
      <h2 class="flex items-center gap-2 text-lg font-semibold text-ink">
        <Code2 size={16} /> 练习代码
      </h2>
      <div class="flex items-center gap-2">
        <select class="input !w-auto py-1 text-xs" bind:value={practiceLang}>
          <option value="go">Go</option>
          <option value="rust">Rust</option>
          <option value="python">Python</option>
        </select>
        <button class="btn-ghost text-xs" on:click={resetPractice}>
          <RotateCcw size={13} /> 重置
        </button>
      </div>
    </div>
    <textarea
      class="min-h-[22rem] w-full resize-y rounded-lg border border-bg-border bg-[#0b0d12] p-4 font-mono text-sm leading-6 text-ink outline-none transition placeholder:text-ink-dim focus:border-accent focus:ring-2 focus:ring-accent/30"
      bind:value={practiceCode}
      on:input={savePractice}
      spellcheck="false"
      placeholder="在这里手写解法，切换题目或语言会自动保存。"
    ></textarea>
  </section>

  <section class="mt-6">
    <h2 class="mb-3 text-lg font-semibold text-ink">站内原创题解</h2>
    {#if p.related_articles.length === 0}
      <div class="card p-6 text-center text-sm text-ink-mute">
        <BookOpen size={20} class="mx-auto mb-2 text-ink-dim" />
        这道题暂无站内原创题解。
      </div>
    {:else}
      <ul class="space-y-2">
        {#each p.related_articles as a}
          <li>
            <a href="/articles/{a.slug}" class="card flex items-start gap-3 p-4 transition hover:border-accent/40">
              <div class="grid h-8 w-8 shrink-0 place-items-center rounded-md bg-accent/10 text-accent ring-1 ring-accent/20">
                <BookOpen size={14} />
              </div>
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2">
                  <span class="chip chip-tag">{a.category}</span>
                  <span class="font-medium text-ink">{a.title}</span>
                </div>
                <div class="mt-1 text-xs text-ink-mute">{a.summary}</div>
              </div>
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  {#if p.has_statement && p.statement_source}
    <footer class="mt-8 rounded-md border border-bg-border bg-bg-soft/40 p-3 text-xs text-ink-mute">
      题面与参考代码来源 <span>{p.statement_source}</span>
      ，依据 <strong>{p.statement_license}</strong> 许可在本站再发布；本站对应于该许可的署名义务。
    </footer>
  {/if}
</div>
