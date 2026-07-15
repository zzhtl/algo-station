<script lang="ts">
  import { BookOpen, Check, ChevronDown, Circle, Clock3, LockOpen, Play, Sparkles } from 'lucide-svelte';
  import type { CurriculumResponse, LessonStatus } from '$lib/api';

  export let data: { curriculum: CurriculumResponse | null };
  const curriculum = data.curriculum;
  let activeStage =
    curriculum?.stages.find((stage) => stage.completed_lessons < stage.lessons.length)?.id ??
    curriculum?.stages[0]?.id ??
    '';

  function statusText(status: LessonStatus) {
    if (status === 'completed') return '已完成';
    if (status === 'in_progress') return '进行中';
    return '未开始';
  }
</script>

<svelte:head><title>学习路线 · Algo Station</title></svelte:head>

<div class="mx-auto max-w-5xl px-5 py-8 sm:px-7 lg:py-12">
  <header class="mb-8">
    <div class="mb-2 flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-accent"><Sparkles size={14} /> 渐进式路线</div>
    <h1 class="text-3xl font-bold tracking-tight text-ink">从基本模式到综合算法</h1>
    <p class="mt-2 max-w-2xl text-sm leading-6 text-ink-mute">12 个阶段按认知依赖排序。前置课是建议而非门锁，你可以随时进入任何课程，再回来补齐基础。</p>
  </header>

  {#if curriculum}
    <section class="mb-7 grid grid-cols-2 gap-3 sm:grid-cols-4">
      <div class="card p-4"><div class="text-xs text-ink-dim">路线进度</div><div class="mt-1 text-2xl font-bold text-accent">{curriculum.summary.completion_percent}%</div></div>
      <div class="card p-4"><div class="text-xs text-ink-dim">课程</div><div class="mt-1 text-2xl font-bold text-ink">{curriculum.summary.completed_lessons}<span class="text-sm font-normal text-ink-dim"> / {curriculum.summary.lesson_count}</span></div></div>
      <div class="card p-4"><div class="text-xs text-ink-dim">配套练习</div><div class="mt-1 text-2xl font-bold text-ink">{curriculum.summary.exercise_count}</div></div>
      <div class="card p-4"><div class="text-xs text-ink-dim">动态图解</div><div class="mt-1 text-2xl font-bold text-ink">{curriculum.summary.visualization_count}</div></div>
    </section>

    <div class="space-y-3">
      {#each curriculum.stages as stage}
        {@const open = activeStage === stage.id}
        <section class="card overflow-hidden" id={stage.id}>
          <button
            class="flex w-full items-center gap-4 p-4 text-left sm:p-5"
            on:click={() => (activeStage = open ? '' : stage.id)}
            aria-expanded={open}
          >
            <div class="grid h-10 w-10 shrink-0 place-items-center rounded-xl bg-accent/10 font-mono text-sm font-bold text-accent">{String(stage.order).padStart(2, '0')}</div>
            <div class="min-w-0 flex-1">
              <div class="flex flex-wrap items-center gap-x-3 gap-y-1">
                <h2 class="font-semibold text-ink">{stage.title}</h2>
                <span class="text-xs tabular-nums text-ink-dim">{stage.completed_lessons}/{stage.lessons.length} 课</span>
              </div>
              <p class="mt-1 line-clamp-1 text-sm text-ink-mute">{stage.description}</p>
            </div>
            <div class="hidden w-28 sm:block">
              <div class="h-1.5 overflow-hidden rounded-full bg-bg-soft"><div class="h-full rounded-full bg-accent" style:width={`${stage.lessons.length ? (stage.completed_lessons / stage.lessons.length) * 100 : 0}%`}></div></div>
            </div>
            <ChevronDown size={18} class="shrink-0 text-ink-dim transition {open ? 'rotate-180' : ''}" />
          </button>

          {#if open}
            <div class="border-t border-bg-border bg-bg-soft/25 p-3 sm:p-4">
              <ol class="grid gap-2">
                {#each stage.lessons as lesson, index}
                  <li>
                    <a href="/learn/{lesson.slug}" class="group flex items-start gap-3 rounded-lg border border-transparent bg-bg-card px-3 py-3 transition hover:border-accent/25 sm:px-4">
                      <div class="mt-0.5 grid h-6 w-6 shrink-0 place-items-center rounded-full {lesson.status === 'completed' ? 'bg-easy/15 text-easy' : lesson.status === 'in_progress' ? 'bg-accent/15 text-accent' : 'bg-bg-soft text-ink-dim'}">
                        {#if lesson.status === 'completed'}<Check size={14} />{:else if lesson.status === 'in_progress'}<Play size={12} />{:else}<Circle size={10} />{/if}
                      </div>
                      <div class="min-w-0 flex-1">
                        <div class="flex flex-wrap items-center gap-2">
                          <span class="text-xs text-ink-dim">{index + 1}</span>
                          <span class="font-medium text-ink group-hover:text-accent">{lesson.title}</span>
                          {#if !lesson.prerequisites_met}<span class="inline-flex items-center gap-1 rounded bg-medium/10 px-1.5 py-0.5 text-[10px] text-medium"><LockOpen size={10} /> 可先学，建议补前置</span>{/if}
                        </div>
                        <p class="mt-1 line-clamp-1 text-xs text-ink-mute">{lesson.summary}</p>
                      </div>
                      <div class="hidden shrink-0 items-center gap-3 text-xs text-ink-dim sm:flex">
                        {#if lesson.has_visualization}<span class="inline-flex items-center gap-1"><Sparkles size={12} /> 图解</span>{/if}
                        <span class="inline-flex items-center gap-1"><BookOpen size={12} /> {lesson.exercise_count} 题</span>
                        <span class="inline-flex items-center gap-1"><Clock3 size={12} /> {lesson.estimated_minutes}m</span>
                        <span class="w-12 text-right">{statusText(lesson.status)}</span>
                      </div>
                    </a>
                  </li>
                {/each}
              </ol>
            </div>
          {/if}
        </section>
      {/each}
    </div>
  {:else}
    <div class="card p-8 text-center text-sm text-ink-mute">路线数据暂时不可用，请确认后端已启动。</div>
  {/if}
</div>
