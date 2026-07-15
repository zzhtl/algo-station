<script lang="ts">
  import {
    AlertTriangle,
    ArrowLeft,
    Check,
    CheckCircle2,
    Circle,
    Clock3,
    Code2,
    Save,
    Sparkles,
    Target
  } from 'lucide-svelte';
  import AlgorithmVisualizer from '$lib/components/AlgorithmVisualizer.svelte';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import MarkdownView from '$lib/components/MarkdownView.svelte';
  import {
    api,
    type ArticleFull,
    type LessonDetailResponse,
    type QuizAttemptResponse
  } from '$lib/api';

  export let data: { lesson: LessonDetailResponse | null; article: ArticleFull | null };
  let detail = data.lesson;
  const article = data.article;
  let answers: number[] = detail ? Array(detail.lesson.quiz.length).fill(-1) : [];
  let quizResult: QuizAttemptResponse | null = null;
  let quizBusy = false;
  let note = detail?.progress.note ?? '';
  let noteState = '';

  async function markAnimationComplete() {
    if (!detail || detail.progress.animation_completed) return;
    const progress = await api.patchLessonProgress(detail.lesson.slug, { animation_completed: true });
    detail = { ...detail, progress };
  }

  async function submitQuiz() {
    if (!detail || answers.some((answer) => answer < 0)) return;
    quizBusy = true;
    try {
      quizResult = await api.submitQuiz(detail.lesson.slug, answers);
      detail = { ...detail, progress: quizResult.progress };
    } finally {
      quizBusy = false;
    }
  }

  async function saveNote() {
    if (!detail) return;
    noteState = '保存中…';
    try {
      const progress = await api.patchLessonProgress(detail.lesson.slug, { note });
      detail = { ...detail, progress };
      noteState = '已保存';
    } catch {
      noteState = '保存失败';
    }
  }
</script>

<svelte:head><title>{detail?.lesson.title ?? '课程'} · Algo Station</title></svelte:head>

<div class="mx-auto max-w-7xl px-5 py-6 sm:px-7 lg:py-9">
  {#if detail}
    <a href="/roadmap#{detail.lesson.stage_id}" class="mb-5 inline-flex items-center gap-1.5 text-sm text-ink-mute hover:text-accent"><ArrowLeft size={15} /> 返回学习路线</a>

    <div class="grid gap-7 xl:grid-cols-[minmax(0,1fr)_19rem]">
      <main class="min-w-0">
        <header class="mb-7">
          <div class="flex flex-wrap items-center gap-2 text-xs text-ink-dim">
            <span class="rounded bg-accent/10 px-2 py-1 font-medium text-accent">{detail.lesson.stage_id.replace('stage-', '阶段 ')}</span>
            <span class="inline-flex items-center gap-1"><Clock3 size={12} /> {detail.lesson.estimated_minutes} 分钟</span>
            <span>{detail.exercises.length} 道配套题</span>
          </div>
          <h1 class="mt-3 text-3xl font-bold tracking-tight text-ink">{detail.lesson.title}</h1>
          <p class="mt-3 max-w-3xl text-sm leading-7 text-ink-mute">{detail.lesson.summary}</p>
        </header>

        {#if !detail.prerequisites_met}
          <div class="mb-6 flex gap-3 rounded-xl border border-medium/25 bg-medium/5 p-4 text-sm text-ink-mute">
            <AlertTriangle size={18} class="mt-0.5 shrink-0 text-medium" />
            <div><span class="font-medium text-ink">建议先补前置课程。</span> 这里不会锁住内容；如果当前概念陌生，可以从路线中回看上一节。</div>
          </div>
        {/if}

        <section class="card mb-7 p-5 sm:p-6">
          <div class="mb-4 flex items-center gap-2"><Target size={17} class="text-accent" /><h2 class="font-semibold text-ink">完成本节后，你应该能</h2></div>
          <ul class="grid gap-2 sm:grid-cols-2">
            {#each detail.lesson.objectives as objective}
              <li class="flex items-start gap-2 text-sm leading-6 text-ink-mute"><Check size={15} class="mt-1 shrink-0 text-easy" /> {objective}</li>
            {/each}
          </ul>
        </section>

        {#if article}
          <article class="card mb-8 p-5 sm:p-8">
            <MarkdownView source={article.content} />
          </article>
        {/if}

        {#if detail.visualization}
          <section class="mb-8" id="visualization">
            <AlgorithmVisualizer
              visualizationId={detail.visualization.id}
              completed={detail.progress.animation_completed}
              on:complete={markAnimationComplete}
            />
          </section>
        {/if}

        <section class="card mb-8 overflow-hidden" id="quiz">
          <div class="border-b border-bg-border p-5 sm:p-6">
            <div class="flex items-center gap-2"><CheckCircle2 size={18} class="text-accent" /><h2 class="text-lg font-semibold text-ink">理解检查</h2></div>
            <p class="mt-1 text-sm text-ink-mute">至少 80 分才算通过。提交后会显示原因，不限制重试次数。</p>
          </div>
          <div class="space-y-7 p-5 sm:p-6">
            {#each detail.lesson.quiz as question, questionIndex}
              {@const correction = quizResult?.corrections.find((item) => item.question_id === question.id)}
              <fieldset>
                <legend class="mb-3 text-sm font-medium leading-6 text-ink">{questionIndex + 1}. {question.prompt}</legend>
                <div class="grid gap-2 sm:grid-cols-2">
                  {#each question.options as option, optionIndex}
                    <label class="flex cursor-pointer items-start gap-2 rounded-lg border p-3 text-sm transition {correction && optionIndex === correction.correct_index ? 'border-easy/50 bg-easy/5' : answers[questionIndex] === optionIndex ? 'border-accent/50 bg-accent/5' : 'border-bg-border hover:border-accent/25'}">
                      <input class="mt-0.5 accent-accent" type="radio" name={question.id} value={optionIndex} bind:group={answers[questionIndex]} disabled={quizBusy} />
                      <span class="text-ink-mute">{option}</span>
                    </label>
                  {/each}
                </div>
                {#if correction}
                  <div class="mt-2 rounded-lg bg-bg-soft p-3 text-xs leading-5 {correction.correct ? 'text-easy' : 'text-medium'}">
                    {correction.correct ? '回答正确。' : '这次没有选对。'} {correction.explanation}
                  </div>
                {/if}
              </fieldset>
            {/each}
            <div class="flex flex-wrap items-center gap-3 border-t border-bg-border pt-5">
              <button class="btn-primary" on:click={submitQuiz} disabled={quizBusy || answers.some((answer) => answer < 0)}>{quizBusy ? '提交中…' : '提交答案'}</button>
              {#if quizResult}<span class="text-sm font-medium {quizResult.passed ? 'text-easy' : 'text-medium'}">本次 {quizResult.score} 分 · {quizResult.passed ? '通过' : '再理解一下'}</span>{/if}
            </div>
          </div>
        </section>

        <section class="mb-8" id="exercises">
          <div class="mb-3 flex items-end justify-between"><div><h2 class="text-lg font-semibold text-ink">配套练习</h2><p class="mt-1 text-xs text-ink-dim">核心题至少一次 Accepted 才能完成本节。</p></div></div>
          <div class="grid gap-3 sm:grid-cols-2">
            {#each detail.exercises as exercise}
              <a href="/practice/{exercise.slug}" class="card group p-4 transition hover:border-accent/35">
                <div class="flex items-start justify-between gap-3">
                  <div class="min-w-0"><div class="flex items-center gap-2"><span class="truncate font-medium text-ink group-hover:text-accent">{exercise.title}</span>{#if exercise.core}<span class="rounded bg-accent/10 px-1.5 py-0.5 text-[10px] text-accent">核心</span>{/if}</div><div class="mt-1 text-xs text-ink-dim">LeetCode #{exercise.problem_id}</div></div>
                  {#if exercise.accepted}<CheckCircle2 size={18} class="shrink-0 text-easy" />{:else}<DifficultyChip difficulty={exercise.difficulty} />{/if}
                </div>
                <p class="mt-3 line-clamp-2 text-xs leading-5 text-ink-mute">{exercise.summary}</p>
              </a>
            {/each}
          </div>
        </section>

        <section class="card p-5 sm:p-6">
          <div class="flex items-center gap-2"><Save size={17} class="text-accent" /><h2 class="font-semibold text-ink">记下你的识别信号</h2></div>
          <p class="mt-1 text-xs text-ink-dim">用自己的话写状态、不变量、边界或下次看到同类题的第一反应。</p>
          <textarea class="input mt-4 min-h-28 resize-y" bind:value={note} placeholder="例如：当左右移动能单调改变答案时，优先考虑双指针……"></textarea>
          <div class="mt-3 flex items-center gap-3"><button class="btn-primary" on:click={saveNote}><Save size={14} /> 保存笔记</button><span class="text-xs text-ink-dim">{noteState}</span></div>
        </section>
      </main>

      <aside class="hidden xl:block">
        <div class="sticky top-7 space-y-4">
          <section class="card p-5">
            <div class="mb-4 flex items-center justify-between"><h2 class="text-sm font-semibold text-ink">完成条件</h2><span class="text-xs {detail.progress.status === 'completed' ? 'text-easy' : 'text-ink-dim'}">{detail.progress.status === 'completed' ? '已完成' : '进行中'}</span></div>
            <ul class="space-y-3 text-sm">
              <li class="flex items-center gap-2 {detail.progress.quiz_best_score >= 80 ? 'text-easy' : 'text-ink-mute'}">{#if detail.progress.quiz_best_score >= 80}<CheckCircle2 size={16} />{:else}<Circle size={15} />{/if} 小测 ≥80 分 <span class="ml-auto text-xs">{detail.progress.quiz_best_score}</span></li>
              <li class="flex items-center gap-2 {detail.progress.core_exercise_accepted ? 'text-easy' : 'text-ink-mute'}">{#if detail.progress.core_exercise_accepted}<CheckCircle2 size={16} />{:else}<Circle size={15} />{/if} 核心题通过</li>
              {#if detail.visualization}<li class="flex items-center gap-2 {detail.progress.animation_completed ? 'text-easy' : 'text-ink-mute'}">{#if detail.progress.animation_completed}<CheckCircle2 size={16} />{:else}<Circle size={15} />{/if} 动图走到结尾</li>{/if}
            </ul>
          </section>
          <nav class="card p-3 text-sm">
            <a class="flex items-center gap-2 rounded-md px-3 py-2 text-ink-mute hover:bg-bg-soft hover:text-ink" href="#quiz"><CheckCircle2 size={15} /> 理解检查</a>
            {#if detail.visualization}<a class="flex items-center gap-2 rounded-md px-3 py-2 text-ink-mute hover:bg-bg-soft hover:text-ink" href="#visualization"><Sparkles size={15} /> 动态图解</a>{/if}
            <a class="flex items-center gap-2 rounded-md px-3 py-2 text-ink-mute hover:bg-bg-soft hover:text-ink" href="#exercises"><Code2 size={15} /> 配套练习</a>
          </nav>
        </div>
      </aside>
    </div>
  {:else}
    <div class="card mx-auto mt-16 max-w-xl p-8 text-center"><h1 class="text-lg font-semibold text-ink">课程不存在或暂时无法加载</h1><a class="btn-primary mt-5" href="/roadmap">返回学习路线</a></div>
  {/if}
</div>
