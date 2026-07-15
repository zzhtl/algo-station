<script lang="ts">
  import { ArrowRight, Brain, CalendarClock, CheckCircle2, Clock3, RefreshCw } from 'lucide-svelte';
  import { api, type ReviewItem, type ReviewRating } from '$lib/api';

  export let data: { reviews: ReviewItem[] };
  let reviews = data.reviews;
  let busySlug = '';
  let feedback = '';
  $: due = reviews.filter((item) => item.due);
  $: upcoming = reviews.filter((item) => !item.due);

  const ratings: { value: ReviewRating; label: string; hint: string; classes: string }[] = [
    { value: 'forgotten', label: '忘记了', hint: '明天再见', classes: 'border-hard/30 text-hard hover:bg-hard/5' },
    { value: 'fuzzy', label: '有点模糊', hint: '缩短间隔', classes: 'border-medium/30 text-medium hover:bg-medium/5' },
    { value: 'remembered', label: '记得很清楚', hint: '进入下一间隔', classes: 'border-easy/30 text-easy hover:bg-easy/5' }
  ];

  async function rate(item: ReviewItem, rating: ReviewRating) {
    busySlug = item.lesson_slug;
    feedback = '';
    try {
      const updated = await api.submitReview(item.lesson_slug, rating);
      reviews = reviews.map((current) => current.lesson_slug === updated.lesson_slug ? updated : current);
      feedback = `“${item.title}”已安排到 ${updated.due_at.slice(0, 10)} 复习`;
    } finally {
      busySlug = '';
    }
  }
</script>

<svelte:head><title>间隔复习 · Algo Station</title></svelte:head>

<div class="mx-auto max-w-5xl px-5 py-8 sm:px-7 lg:py-12">
  <header class="mb-8"><div class="mb-2 flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-accent"><Brain size={14} /> 间隔复习</div><h1 class="text-3xl font-bold tracking-tight text-ink">在快要忘记时，再想一次</h1><p class="mt-2 max-w-2xl text-sm leading-6 text-ink-mute">课程完成后按 1、3、7、14、30 天安排复习。评价真实记忆状态，系统会自动调整下一次间隔。</p></header>

  <section class="mb-7 grid gap-3 sm:grid-cols-3">
    <div class="card p-4"><div class="text-xs text-ink-dim">今日到期</div><div class="mt-1 text-2xl font-bold text-accent">{due.length}</div></div>
    <div class="card p-4"><div class="text-xs text-ink-dim">已进入复习</div><div class="mt-1 text-2xl font-bold text-ink">{reviews.length}</div></div>
    <div class="card p-4"><div class="text-xs text-ink-dim">长期巩固</div><div class="mt-1 text-2xl font-bold text-easy">{reviews.filter((item) => item.mastered).length}</div></div>
  </section>

  {#if feedback}<div class="mb-5 flex items-center gap-2 rounded-lg border border-easy/25 bg-easy/5 px-4 py-3 text-sm text-easy"><CheckCircle2 size={16} /> {feedback}</div>{/if}

  <section>
    <div class="mb-3 flex items-center gap-2"><RefreshCw size={17} class="text-accent" /><h2 class="text-lg font-semibold text-ink">现在复习</h2></div>
    {#if due.length}
      <div class="space-y-3">
        {#each due as item}
          <article class="card p-5 sm:p-6">
            <div class="flex flex-col justify-between gap-4 sm:flex-row sm:items-start">
              <div><div class="flex flex-wrap items-center gap-2"><h3 class="font-semibold text-ink">{item.title}</h3><span class="rounded bg-accent/10 px-2 py-0.5 text-[10px] text-accent">第 {item.step + 1} 轮</span></div><p class="mt-2 text-sm leading-6 text-ink-mute">先不看答案：说出识别信号、核心不变量、一个边界，再回课程快速核对。</p><a href="/learn/{item.lesson_slug}" class="mt-2 inline-flex items-center gap-1 text-xs text-accent hover:underline">打开课程核对 <ArrowRight size={12} /></a></div>
              <div class="shrink-0 text-xs text-ink-dim">到期 {item.due_at.slice(0, 10)}</div>
            </div>
            <div class="mt-5 grid gap-2 border-t border-bg-border pt-4 sm:grid-cols-3">
              {#each ratings as rating}
                <button class="rounded-lg border px-3 py-3 text-left transition {rating.classes}" on:click={() => rate(item, rating.value)} disabled={busySlug === item.lesson_slug}><span class="block text-sm font-medium">{rating.label}</span><span class="mt-0.5 block text-[10px] opacity-70">{rating.hint}</span></button>
              {/each}
            </div>
          </article>
        {/each}
      </div>
    {:else}
      <div class="card p-8 text-center"><CheckCircle2 size={24} class="mx-auto text-easy" /><h3 class="mt-3 font-medium text-ink">今天的复习已完成</h3><p class="mt-1 text-sm text-ink-mute">保持轻量即可，下次到期再回来。</p></div>
    {/if}
  </section>

  {#if upcoming.length}
    <section class="mt-8"><div class="mb-3 flex items-center gap-2"><CalendarClock size={17} class="text-ink-dim" /><h2 class="text-lg font-semibold text-ink">后续安排</h2></div><div class="card divide-y divide-bg-border">{#each upcoming.slice(0, 12) as item}<div class="flex items-center gap-3 px-4 py-3 text-sm"><Clock3 size={14} class="shrink-0 text-ink-dim" /><a href="/learn/{item.lesson_slug}" class="min-w-0 flex-1 truncate text-ink-mute hover:text-accent">{item.title}</a><span class="shrink-0 text-xs text-ink-dim">{item.due_at.slice(0, 10)}</span>{#if item.mastered}<span class="rounded bg-easy/10 px-1.5 py-0.5 text-[10px] text-easy">月度巩固</span>{/if}</div>{/each}</div></section>
  {/if}
</div>

