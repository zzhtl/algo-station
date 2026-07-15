<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import {
    AlertCircle,
    ArrowLeft,
    CheckCircle2,
    Clock3,
    Code2,
    LoaderCircle,
    Play,
    Save,
    ServerOff,
    TerminalSquare,
    XCircle
  } from 'lucide-svelte';
  import CodeEditor from '$lib/components/CodeEditor.svelte';
  import DifficultyChip from '$lib/components/DifficultyChip.svelte';
  import MarkdownView from '$lib/components/MarkdownView.svelte';
  import {
    api,
    type ExerciseContract,
    type ExerciseDetailResponse,
    type JudgeStatusResponse,
    type Language,
    type Submission,
    type SubmissionListResponse,
    type SubmissionStatus,
    type StatementResponse
  } from '$lib/api';

  export let data: {
    exercise: ExerciseDetailResponse | null;
    judge: JudgeStatusResponse | null;
    submissions: SubmissionListResponse | null;
    statement: StatementResponse | null;
  };
  const exercise = data.exercise;
  const judge = data.judge;
  let language: Language = 'rust';
  let contract: ExerciseContract = 'function';
  let code = exercise?.starters.find((item) => item.language === language && item.contract === contract)?.code ?? '';
  let history = data.submissions?.items ?? [];
  let current: Submission | null = history[0] ?? null;
  let saveState = '';
  let submitting = false;
  let saveTimer: ReturnType<typeof setTimeout> | undefined;
  let pollTimer: ReturnType<typeof setTimeout> | undefined;

  onMount(loadDraft);
  onDestroy(() => {
    if (saveTimer) clearTimeout(saveTimer);
    if (pollTimer) clearTimeout(pollTimer);
  });

  function starter(nextLanguage = language, nextContract = contract) {
    return exercise?.starters.find((item) => item.language === nextLanguage && item.contract === nextContract)?.code ?? '';
  }

  async function loadDraft() {
    if (!exercise) return;
    try {
      const draft = await api.exerciseDraft(exercise.slug, language, contract);
      code = draft?.code || starter();
      saveState = draft ? '已恢复草稿' : '';
    } catch {
      code = starter();
    }
  }

  async function selectTarget(nextLanguage: Language, nextContract: ExerciseContract) {
    if (!exercise || (nextLanguage === language && nextContract === contract)) return;
    await saveDraft();
    language = nextLanguage;
    contract = nextContract;
    await loadDraft();
  }

  function codeChanged(value: string) {
    code = value;
    saveState = '未保存';
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(saveDraft, 900);
  }

  async function saveDraft() {
    if (!exercise || !code.trim() || code.length > 65_536) return;
    if (saveTimer) clearTimeout(saveTimer);
    saveState = '保存中…';
    try {
      await api.putExerciseDraft(exercise.slug, language, contract, code);
      saveState = '草稿已保存';
    } catch {
      saveState = '保存失败';
    }
  }

  async function submit() {
    if (!exercise || submitting || !code.trim() || code.length > 65_536) return;
    submitting = true;
    await saveDraft();
    try {
      current = await api.createSubmission(exercise.slug, language, contract, code);
      history = [current, ...history.filter((item) => item.id !== current?.id)];
      schedulePoll();
    } finally {
      submitting = false;
    }
  }

  function isPending(status: SubmissionStatus) {
    return status === 'queued' || status === 'running';
  }

  function schedulePoll() {
    if (!current || !isPending(current.status)) return;
    if (pollTimer) clearTimeout(pollTimer);
    pollTimer = setTimeout(async () => {
      if (!current) return;
      try {
        current = await api.submission(current.id);
        history = [current, ...history.filter((item) => item.id !== current?.id)];
      } finally {
        if (current && isPending(current.status)) schedulePoll();
      }
    }, 900);
  }

  function statusLabel(status: SubmissionStatus) {
    const labels: Record<SubmissionStatus, string> = {
      queued: '排队中', running: '运行中', accepted: 'Accepted', wrong_answer: '答案错误',
      compile_error: '编译错误', runtime_error: '运行错误', time_limit: '超出时间',
      memory_limit: '超出内存', internal_error: '判题异常'
    };
    return labels[status];
  }
</script>

<svelte:head><title>{exercise?.title ?? '练习'} · 配套训练</title></svelte:head>

<div class="mx-auto max-w-[1500px] px-4 py-5 sm:px-6">
  {#if exercise}
    <a href="/practice" class="mb-4 inline-flex items-center gap-1.5 text-sm text-ink-mute hover:text-accent"><ArrowLeft size={15} /> 返回训练列表</a>
    <header class="mb-5 flex flex-col justify-between gap-4 lg:flex-row lg:items-end">
      <div><div class="flex flex-wrap items-center gap-2"><h1 class="text-2xl font-bold text-ink">{exercise.title}</h1><DifficultyChip difficulty={exercise.difficulty} />{#if exercise.accepted}<span class="inline-flex items-center gap-1 text-xs font-medium text-easy"><CheckCircle2 size={14} /> 已通过</span>{/if}</div><p class="mt-2 max-w-3xl text-sm leading-6 text-ink-mute">{exercise.summary}</p></div>
      <div class="flex items-center gap-3 text-xs text-ink-dim"><span>LeetCode #{exercise.problem_id}</span><span>{exercise.hidden_case_count} 个隐藏用例</span><span>{exercise.limits.memory_mb} MB</span></div>
    </header>

    {#if !judge?.online}
      <div class="mb-4 flex items-center gap-2 rounded-lg border border-medium/25 bg-medium/5 px-4 py-3 text-xs text-ink-mute"><ServerOff size={15} class="text-medium" /> Worker 当前离线；提交会安全进入队列，启动 Worker 后自动处理。</div>
    {/if}

    <div class="grid gap-5 xl:grid-cols-[minmax(20rem,0.72fr)_minmax(34rem,1.28fr)]">
      <div class="space-y-5">
        {#if data.statement}
          <section class="card p-5"><div class="mb-4 flex items-center gap-2"><Code2 size={17} class="text-accent" /><h2 class="font-semibold text-ink">题目说明</h2></div><MarkdownView source={data.statement.content} /></section>
        {/if}
        <section class="card p-5"><div class="mb-3 flex items-center gap-2"><TerminalSquare size={17} class="text-accent" /><h2 class="font-semibold text-ink">输入输出约定</h2></div><p class="text-sm leading-6 text-ink-mute">函数模式会把完整输入字符串传给 <code class="rounded bg-bg-soft px-1 text-accent">{language === 'go' ? 'Solve(input string)' : 'solve(&str)'}</code>；标准输入输出模式直接运行完整程序。输出需与样例一致，行尾空格会忽略。</p></section>
        <section class="card overflow-hidden"><div class="border-b border-bg-border px-5 py-4"><h2 class="font-semibold text-ink">公开样例</h2></div>{#each exercise.public_cases as testCase}<div class="border-b border-bg-border p-5 last:border-b-0"><div class="mb-2 text-xs font-medium text-ink-dim">{testCase.name}</div><div class="grid gap-3 sm:grid-cols-2"><div><div class="mb-1 text-[10px] uppercase tracking-wider text-ink-dim">Input</div><pre class="overflow-x-auto rounded-lg bg-bg-soft p-3 text-xs text-ink-mute">{testCase.input}</pre></div><div><div class="mb-1 text-[10px] uppercase tracking-wider text-ink-dim">Expected</div><pre class="overflow-x-auto rounded-lg bg-bg-soft p-3 text-xs text-ink-mute">{testCase.expected}</pre></div></div></div>{/each}</section>
        {#if current}
          <section class="card overflow-hidden">
            <div class="flex items-center justify-between border-b border-bg-border px-5 py-4"><h2 class="font-semibold text-ink">最近结果</h2><span class="text-xs {current.status === 'accepted' ? 'text-easy' : isPending(current.status) ? 'text-accent' : 'text-hard'}">{statusLabel(current.status)}</span></div>
            <div class="p-5">
              {#if isPending(current.status)}
                <div class="flex items-center gap-2 text-sm text-ink-mute"><LoaderCircle size={17} class="animate-spin text-accent" /> {current.status === 'queued' ? '等待 Worker 领取任务…' : '正在隔离容器中编译和运行…'}</div>
              {:else if current.result}
                <div class="flex items-start gap-2 {current.status === 'accepted' ? 'text-easy' : 'text-hard'}">{#if current.status === 'accepted'}<CheckCircle2 size={19} />{:else}<XCircle size={19} />{/if}<div><div class="font-medium">{current.result.message}</div><div class="mt-1 text-xs text-ink-dim">总耗时 {current.result.duration_ms} ms · 第 {current.attempts} 次执行</div></div></div>
                {#if current.result.compile_output}<pre class="mt-4 max-h-44 overflow-auto rounded-lg bg-bg-soft p-3 text-xs text-hard">{current.result.compile_output}</pre>{/if}
                {#if current.result.cases.length}<div class="mt-4 space-y-2">{#each current.result.cases as resultCase}<div class="flex items-center justify-between rounded-lg border border-bg-border px-3 py-2 text-xs"><span class="inline-flex items-center gap-2 text-ink-mute">{#if resultCase.status === 'accepted'}<CheckCircle2 size={14} class="text-easy" />{:else}<AlertCircle size={14} class="text-hard" />{/if}{resultCase.name} · {resultCase.visibility === 'hidden' ? '隐藏' : '公开'}</span><span class="text-ink-dim">{resultCase.duration_ms} ms</span></div>{/each}</div>{/if}
              {/if}
            </div>
          </section>
        {/if}
      </div>

      <div class="min-w-0">
        <section class="card overflow-hidden">
          <div class="flex flex-wrap items-center justify-between gap-3 border-b border-bg-border bg-bg-soft/35 px-4 py-3">
            <div class="flex items-center gap-1 rounded-lg bg-bg-card p-1 ring-1 ring-bg-border">
              <button class="rounded-md px-3 py-1.5 text-xs font-medium {language === 'rust' ? 'bg-accent/15 text-accent' : 'text-ink-mute'}" on:click={() => selectTarget('rust', contract)}>Rust</button>
              <button class="rounded-md px-3 py-1.5 text-xs font-medium {language === 'go' ? 'bg-accent/15 text-accent' : 'text-ink-mute'}" on:click={() => selectTarget('go', contract)}>Go</button>
            </div>
            <div class="flex items-center gap-1 rounded-lg bg-bg-card p-1 ring-1 ring-bg-border">
              <button class="rounded-md px-3 py-1.5 text-xs font-medium {contract === 'function' ? 'bg-accent/15 text-accent' : 'text-ink-mute'}" on:click={() => selectTarget(language, 'function')}>函数</button>
              <button class="rounded-md px-3 py-1.5 text-xs font-medium {contract === 'stdio' ? 'bg-accent/15 text-accent' : 'text-ink-mute'}" on:click={() => selectTarget(language, 'stdio')}>标准输入输出</button>
            </div>
            <div class="flex items-center gap-3 text-xs text-ink-dim"><span>{code.length.toLocaleString()} / 65,536</span><span>{saveState}</span><button class="btn-ghost py-1 text-xs" on:click={saveDraft}><Save size={13} /> 保存</button></div>
          </div>
          <div class="p-3"><CodeEditor value={code} lang={language} placeholder="在这里完成解法…" on:change={(event) => codeChanged(event.detail)} /></div>
          <div class="flex flex-wrap items-center justify-between gap-3 border-t border-bg-border px-4 py-3">
            <div class="text-xs text-ink-dim"><Clock3 size={12} class="mr-1 inline" /> 单例 {exercise.limits.case_ms} ms · 总计 {exercise.limits.total_ms} ms</div>
            <button class="btn-primary" on:click={submit} disabled={submitting || !code.trim() || code.length > 65_536}><Play size={15} /> {submitting ? '入队中…' : '提交判题'}</button>
          </div>
        </section>

        {#if history.length}
          <section class="mt-5 card overflow-hidden"><div class="border-b border-bg-border px-5 py-4"><h2 class="font-semibold text-ink">提交记录</h2></div><div class="divide-y divide-bg-border">{#each history as submission}<button class="grid w-full grid-cols-[minmax(0,1fr)_7rem_6rem] items-center gap-3 px-5 py-3 text-left text-xs hover:bg-bg-soft/40" on:click={() => { current = submission; if (isPending(submission.status)) schedulePoll(); }}><span class="text-ink-mute">#{submission.id} · {submission.language}/{submission.contract}</span><span class="{submission.status === 'accepted' ? 'text-easy' : isPending(submission.status) ? 'text-accent' : 'text-hard'}">{statusLabel(submission.status)}</span><span class="text-right text-ink-dim">{submission.queued_at.slice(5, 16)}</span></button>{/each}</div></section>
        {/if}
      </div>
    </div>
  {:else}
    <div class="card mx-auto mt-16 max-w-lg p-8 text-center"><Code2 size={22} class="mx-auto text-ink-dim" /><h1 class="mt-3 font-semibold text-ink">练习不存在或暂时无法加载</h1><a class="btn-primary mt-5" href="/practice">返回训练列表</a></div>
  {/if}
</div>
