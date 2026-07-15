<script lang="ts">
  import { CheckCircle2, ChevronRight, Code2, Filter, Search, Server, ServerOff } from 'lucide-svelte';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import {
    api,
    type CurriculumResponse,
    type Difficulty,
    type ExerciseListItem,
    type ExerciseListResponse,
    type JudgeStatusResponse
  } from '$lib/api';

  export let data: {
    exercises: ExerciseListResponse | null;
    curriculum: CurriculumResponse | null;
    judge: JudgeStatusResponse | null;
  };
  let items: ExerciseListItem[] = data.exercises?.items ?? [];
  let nextCursor = data.exercises?.next_cursor ?? null;
  let total = data.exercises?.total ?? 0;
  let stageId = '';
  let difficulty: Difficulty | '' = '';
  let status: 'accepted' | 'unstarted' | '' = '';
  let search = '';
  let busy = false;
  const judge = data.judge;
  $: shown = items.filter((item) => item.title.toLowerCase().includes(search.trim().toLowerCase()));

  async function reload() {
    busy = true;
    try {
      const response = await api.exercises({ stage_id: stageId, difficulty, status, limit: 60 });
      items = response.items;
      nextCursor = response.next_cursor;
      total = response.total;
    } finally {
      busy = false;
    }
  }

  async function loadMore() {
    if (nextCursor === null) return;
    busy = true;
    try {
      const response = await api.exercises({ stage_id: stageId, difficulty, status, cursor: nextCursor, limit: 60 });
      items = [...items, ...response.items];
      nextCursor = response.next_cursor;
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>配套训练 · Algo Station</title></svelte:head>

<div class="mx-auto max-w-6xl px-5 py-8 sm:px-7 lg:py-12">
  <header class="mb-7 flex flex-col justify-between gap-4 sm:flex-row sm:items-end">
    <div><div class="mb-2 flex items-center gap-2 text-xs font-semibold uppercase tracking-wider text-accent"><Code2 size={14} /> 配套训练</div><h1 class="text-3xl font-bold tracking-tight text-ink">把理解变成能独立写出的代码</h1><p class="mt-2 max-w-2xl text-sm leading-6 text-ink-mute">150 道课程精选题支持 Go、Rust，以及函数和标准输入输出两种提交方式。</p></div>
    <div class="inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs {judge?.online ? 'border-easy/30 bg-easy/5 text-easy' : 'border-medium/30 bg-medium/5 text-medium'}">
      {#if judge?.online}<Server size={13} /> Worker 在线 · 队列 {judge.queue_size}{:else}<ServerOff size={13} /> Worker 离线{/if}
    </div>
  </header>

  <section class="card mb-5 p-4">
    <div class="grid gap-3 md:grid-cols-[minmax(13rem,1fr)_repeat(3,minmax(9rem,auto))]">
      <label class="relative"><Search size={15} class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-ink-dim" /><input class="input pl-9" placeholder="筛选当前结果" bind:value={search} /></label>
      <label><span class="sr-only">阶段</span><select class="input" bind:value={stageId} on:change={reload}><option value="">全部阶段</option>{#each data.curriculum?.stages ?? [] as stage}<option value={stage.id}>{stage.order}. {stage.title}</option>{/each}</select></label>
      <label><span class="sr-only">难度</span><select class="input" bind:value={difficulty} on:change={reload}><option value="">全部难度</option><option value="Easy">简单</option><option value="Medium">中等</option><option value="Hard">困难</option></select></label>
      <label><span class="sr-only">完成状态</span><select class="input" bind:value={status} on:change={reload}><option value="">全部状态</option><option value="unstarted">待完成</option><option value="accepted">已通过</option></select></label>
    </div>
  </section>

  <div class="mb-3 flex items-center justify-between text-xs text-ink-dim"><span class="inline-flex items-center gap-1"><Filter size={13} /> 共 {total} 题，当前显示 {shown.length}</span>{#if busy}<span>正在更新…</span>{/if}</div>

  {#if shown.length}
    <div class="grid gap-2">
      {#each shown as exercise}
        <a href="/practice/{exercise.slug}" class="card group grid items-center gap-3 p-4 transition hover:border-accent/35 sm:grid-cols-[minmax(0,1fr)_8rem_7rem_2rem]">
          <div class="min-w-0"><div class="flex items-center gap-2"><span class="font-medium text-ink group-hover:text-accent">{exercise.title}</span><span class="text-xs text-ink-dim">#{exercise.problem_id}</span></div><p class="mt-1 line-clamp-1 text-xs text-ink-mute">{exercise.summary}</p></div>
          <div class="text-xs text-ink-dim">{exercise.stage_id.replace('stage-', '阶段 ')}</div>
          <div>{#if exercise.accepted}<span class="inline-flex items-center gap-1 text-xs font-medium text-easy"><CheckCircle2 size={15} /> Accepted</span>{:else}<DifficultyChip difficulty={exercise.difficulty} />{/if}</div>
          <ChevronRight size={16} class="text-ink-dim group-hover:text-accent" />
        </a>
      {/each}
    </div>
    {#if nextCursor !== null}<div class="mt-5 text-center"><button class="btn-ghost" on:click={loadMore} disabled={busy}>加载更多</button></div>{/if}
  {:else}
    <div class="card p-10 text-center text-sm text-ink-mute">没有匹配当前条件的练习。</div>
  {/if}
</div>

