<script lang="ts">
  import {
    ArrowRight,
    BookOpen,
    CalendarCheck,
    CheckCircle2,
    Clock3,
    Flame,
    PlayCircle,
    RefreshCw,
    Route,
    ServerOff,
    Sparkles
  } from 'lucide-svelte';
  import { api, type DashboardResponse } from '$lib/api';

  export let data: { dashboard: DashboardResponse | null };
  let dashboard = data.dashboard;
  let updatingPlan = false;

  async function changeTarget(event: Event) {
    const target = Number((event.currentTarget as HTMLSelectElement).value);
    if (!dashboard || !Number.isFinite(target)) return;
    updatingPlan = true;
    try {
      dashboard = { ...dashboard, daily_plan: await api.putDailyPlan(target) };
    } finally {
      updatingPlan = false;
    }
  }
</script>

<svelte:head><title>Algo Station · 今日学习</title></svelte:head>

<div class="mx-auto max-w-6xl px-5 py-8 sm:px-7 lg:py-12">
  {#if dashboard}
    <header class="mb-8 flex flex-col justify-between gap-5 sm:flex-row sm:items-end">
      <div>
        <div class="mb-2 flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-accent">
          <Sparkles size={14} /> 今日学习
        </div>
        <h1 class="text-3xl font-bold tracking-tight text-ink sm:text-4xl">继续前进一小步</h1>
        <p class="mt-2 max-w-xl text-sm leading-6 text-ink-mute">
          今天安排约 {dashboard.daily_plan.estimated_minutes} 分钟。理解、图解、练习和复习会一起推进，不需要一次学完很多。
        </p>
      </div>
      <div class="flex items-center gap-3 text-sm text-ink-mute">
        <span class="inline-flex items-center gap-1.5"><Flame size={16} class="text-medium" /> 连续 {dashboard.streak_days} 天</span>
        <span class="h-4 w-px bg-bg-border"></span>
        <span>{dashboard.completed_lessons} / {dashboard.total_lessons} 课</span>
      </div>
    </header>

    <section class="grid gap-5 lg:grid-cols-[minmax(0,1.6fr)_minmax(17rem,0.7fr)]">
      <div class="card overflow-hidden">
        <div class="border-b border-bg-border p-5 sm:p-6">
          <div class="flex items-start justify-between gap-4">
            <div>
              <div class="text-xs font-medium text-ink-dim">下一节</div>
              {#if dashboard.next_lesson}
                <h2 class="mt-1 text-xl font-semibold text-ink">{dashboard.next_lesson.title}</h2>
                <p class="mt-2 line-clamp-2 max-w-2xl text-sm leading-6 text-ink-mute">{dashboard.next_lesson.summary}</p>
              {:else}
                <h2 class="mt-1 text-xl font-semibold text-ink">路线已全部完成</h2>
              {/if}
            </div>
            {#if dashboard.next_lesson}
              <div class="shrink-0 rounded-full bg-accent/10 px-3 py-1 text-xs text-accent">
                {dashboard.next_lesson.estimated_minutes} 分钟
              </div>
            {/if}
          </div>
          {#if dashboard.next_lesson}
            <div class="mt-5 flex flex-wrap items-center gap-2">
              <a class="btn-primary" href="/learn/{dashboard.next_lesson.slug}">
                <PlayCircle size={17} /> 继续学习
              </a>
              <a class="btn-ghost" href="/roadmap">查看完整路线 <ArrowRight size={15} /></a>
            </div>
          {/if}
        </div>
        <div class="bg-bg-soft/35 p-5 sm:p-6">
          <div class="mb-3 flex items-center justify-between gap-4">
            <span class="text-sm font-medium text-ink">总路线进度</span>
            <span class="text-sm font-semibold tabular-nums text-accent">{dashboard.completion_percent}%</span>
          </div>
          <div class="h-2.5 overflow-hidden rounded-full bg-bg-border/70" aria-label={`路线完成 ${dashboard.completion_percent}%`}>
            <div class="h-full rounded-full bg-accent transition-[width]" style:width={`${dashboard.completion_percent}%`}></div>
          </div>
        </div>
      </div>

      <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-1">
        <a href="/reviews" class="card group p-5 transition hover:border-accent/35">
          <div class="flex items-center justify-between">
            <div class="grid h-9 w-9 place-items-center rounded-lg bg-medium/10 text-medium"><RefreshCw size={18} /></div>
            <ArrowRight size={16} class="text-ink-dim transition group-hover:translate-x-0.5 group-hover:text-accent" />
          </div>
          <div class="mt-4 text-2xl font-bold text-ink">{dashboard.due_reviews}</div>
          <div class="mt-1 text-sm text-ink-mute">今日待复习</div>
        </a>
        <div class="card p-5">
          <div class="flex items-center justify-between">
            <div class="grid h-9 w-9 place-items-center rounded-lg bg-easy/10 text-easy"><CalendarCheck size={18} /></div>
            <span class="text-xs text-ink-dim">轻量记录</span>
          </div>
          <div class="mt-4 text-2xl font-bold text-ink">{dashboard.streak_days}<span class="ml-1 text-sm font-normal text-ink-mute">天</span></div>
          <div class="mt-1 text-sm text-ink-mute">当前学习连续天数</div>
        </div>
      </div>
    </section>

    <section class="mt-8">
      <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
        <div>
          <h2 class="text-lg font-semibold text-ink">今日计划</h2>
          <p class="mt-0.5 text-xs text-ink-dim">按顺序完成即可，前置课程只是提示，不会锁住内容。</p>
        </div>
        <label class="flex items-center gap-2 text-xs text-ink-mute">
          <Clock3 size={14} /> 每日目标
          <select class="rounded-md border border-bg-border bg-bg-card px-2 py-1.5 text-ink" value={dashboard.daily_plan.target_minutes} on:change={changeTarget} disabled={updatingPlan}>
            <option value="30">30 分钟</option>
            <option value="45">45 分钟</option>
            <option value="60">60 分钟</option>
            <option value="90">90 分钟</option>
          </select>
        </label>
      </div>
      <div class="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
        {#each dashboard.daily_plan.lessons as lesson, index}
          <a href="/learn/{lesson.slug}" class="card group flex gap-4 p-4 transition hover:border-accent/35 hover:-translate-y-0.5">
            <div class="grid h-9 w-9 shrink-0 place-items-center rounded-full border border-bg-border bg-bg-soft text-sm font-semibold text-ink-mute">{index + 1}</div>
            <div class="min-w-0 flex-1">
              <div class="truncate font-medium text-ink group-hover:text-accent">{lesson.title}</div>
              <div class="mt-1 flex items-center gap-3 text-xs text-ink-dim">
                <span>{lesson.estimated_minutes} 分钟</span>
                <span>{lesson.exercise_count} 道练习</span>
                {#if lesson.has_visualization}<span>动态图</span>{/if}
              </div>
            </div>
          </a>
        {/each}
      </div>
    </section>

    {#if !dashboard.judge_online}
      <div class="mt-7 flex items-start gap-3 rounded-xl border border-medium/25 bg-medium/5 p-4 text-sm text-ink-mute">
        <ServerOff size={18} class="mt-0.5 shrink-0 text-medium" />
        <div><span class="font-medium text-ink">判题 Worker 未在线。</span> 仍可阅读、播放图解和保存代码；提交会排队，Worker 启动后自动处理。</div>
      </div>
    {/if}

    <section class="mt-8 grid gap-3 sm:grid-cols-3">
      <a href="/roadmap" class="card flex items-center gap-3 p-4 text-sm text-ink-mute transition hover:border-accent/30 hover:text-ink"><Route size={18} class="text-accent" /> 12 阶段学习路线</a>
      <a href="/practice" class="card flex items-center gap-3 p-4 text-sm text-ink-mute transition hover:border-accent/30 hover:text-ink"><CheckCircle2 size={18} class="text-accent" /> 150 道配套训练</a>
      <a href="/library" class="card flex items-center gap-3 p-4 text-sm text-ink-mute transition hover:border-accent/30 hover:text-ink"><BookOpen size={18} class="text-accent" /> 336 篇资料库</a>
    </section>
  {:else}
    <div class="mx-auto mt-24 max-w-lg text-center">
      <div class="mx-auto grid h-12 w-12 place-items-center rounded-xl bg-bg-soft text-ink-dim"><ServerOff size={22} /></div>
      <h1 class="mt-4 text-xl font-semibold text-ink">暂时无法读取学习数据</h1>
      <p class="mt-2 text-sm text-ink-mute">请确认后端已经启动，然后刷新页面。你的本地数据库不会受到影响。</p>
      <button class="btn-primary mt-5" on:click={() => location.reload()}><RefreshCw size={15} /> 重新加载</button>
    </div>
  {/if}
</div>

