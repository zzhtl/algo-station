<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount } from 'svelte';
  import { ChevronLeft, ChevronRight, Pause, Play, RotateCcw } from 'lucide-svelte';
  import { getVisualization, type CellState, type VisualNode } from '$lib/visualizations';

  export let visualizationId: string;
  export let completed = false;

  const dispatch = createEventDispatcher<{ complete: void }>();
  let frameIndex = 0;
  let playing = false;
  let speed = 1100;
  let reduceMotion = false;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let completionSent = completed;
  let activeVisualizationId = visualizationId;

  $: if (visualizationId !== activeVisualizationId) {
    stop();
    activeVisualizationId = visualizationId;
    frameIndex = 0;
    completionSent = completed;
  }
  $: spec = getVisualization(visualizationId);
  $: frame = spec?.frames[frameIndex];
  $: lastIndex = Math.max(0, (spec?.frames.length ?? 1) - 1);
  $: if (spec && frameIndex === lastIndex && !completionSent) {
    completionSent = true;
    dispatch('complete');
  }

  onMount(() => {
    reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  });

  onDestroy(stop);

  function stateClass(state: CellState) {
    return `state-${state}`;
  }

  function nodeById(id: string): VisualNode | undefined {
    return frame?.nodes?.find((node) => node.id === id);
  }

  function previous() {
    stop();
    frameIndex = Math.max(0, frameIndex - 1);
  }

  function next() {
    frameIndex = Math.min(lastIndex, frameIndex + 1);
    if (frameIndex === lastIndex) stop();
  }

  function reset() {
    stop();
    frameIndex = 0;
  }

  function stop() {
    playing = false;
    if (timer) clearTimeout(timer);
    timer = undefined;
  }

  function schedule() {
    if (!playing) return;
    timer = setTimeout(() => {
      if (frameIndex >= lastIndex) {
        stop();
        return;
      }
      frameIndex += 1;
      schedule();
    }, speed);
  }

  function togglePlay() {
    if (playing) {
      stop();
      return;
    }
    if (frameIndex >= lastIndex) frameIndex = 0;
    playing = true;
    schedule();
  }

</script>

{#if spec && frame}
  <section
    class="visualizer card overflow-hidden"
    aria-label={`${spec.title}动态图解`}
  >
    <header class="flex flex-wrap items-start justify-between gap-3 border-b border-bg-border px-5 py-4">
      <div>
        <div class="text-xs font-semibold uppercase tracking-wider text-accent">动态图解</div>
        <h3 class="mt-1 text-lg font-semibold text-ink">{spec.title}</h3>
      </div>
      <div class="rounded-full bg-bg-soft px-3 py-1 text-xs tabular-nums text-ink-mute">
        {frameIndex + 1} / {spec.frames.length}
      </div>
    </header>

    <div class="grid gap-0 lg:grid-cols-[minmax(0,1fr)_18rem]">
      <div class="min-h-[22rem] p-5 sm:p-7" aria-live="polite">
        <div class="mb-6">
          <h4 class="text-base font-semibold text-ink">{frame.title}</h4>
          <p class="mt-1 text-sm leading-6 text-ink-mute">{frame.description}</p>
        </div>

        {#if frame.cells}
          <div class="flex min-h-36 items-center justify-center overflow-x-auto py-8">
            <div class="flex min-w-max items-end gap-2">
              {#each frame.cells as cell, index}
                <div class="relative flex flex-col items-center">
                  <div class="absolute -top-8 flex min-w-max flex-col items-center gap-0.5">
                    {#each frame.pointers?.filter((pointer) => pointer.index === index) ?? [] as pointer}
                      <span class="text-[11px] font-semibold text-accent">{pointer.label} ↓</span>
                    {/each}
                  </div>
                  <div class="visual-cell {stateClass(cell.state)}">{cell.value}</div>
                  <span class="mt-2 font-mono text-[10px] text-ink-dim">{cell.label}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        {#if frame.nodes && frame.edges}
          <div class="mx-auto h-64 w-full max-w-2xl rounded-lg bg-bg-soft/60 p-2">
            <svg viewBox="0 0 100 100" class="h-full w-full" role="img" aria-label={frame.title}>
              {#each frame.edges as edge}
                {@const from = nodeById(edge.from)}
                {@const to = nodeById(edge.to)}
                {#if from && to}
                  <line
                    x1={from.x}
                    y1={from.y}
                    x2={to.x}
                    y2={to.y}
                    class="visual-edge {stateClass(edge.state)}"
                  />
                  {#if edge.label}
                    <text x={(from.x + to.x) / 2} y={(from.y + to.y) / 2 - 2} class="edge-label">
                      {edge.label}
                    </text>
                  {/if}
                {/if}
              {/each}
              {#each frame.nodes as node}
                <g class="visual-node {stateClass(node.state)}">
                  <circle cx={node.x} cy={node.y} r={node.label.length > 5 ? 7.5 : 5.5} />
                  <text x={node.x} y={node.y + 1.5} text-anchor="middle">{node.label}</text>
                </g>
              {/each}
            </svg>
          </div>
        {/if}

        {#if frame.table}
          <div class="overflow-x-auto py-5">
            <table class="mx-auto border-separate border-spacing-1 text-center text-sm">
              <thead>
                <tr>
                  <th class="px-3 py-2 text-ink-dim">状态</th>
                  {#each frame.table.columns as column}
                    <th class="min-w-12 rounded bg-bg-soft px-3 py-2 font-mono text-ink-mute">{column}</th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each frame.table.rows as row, rowIndex}
                  <tr>
                    <th class="px-3 py-2 text-ink-mute">{row.label}</th>
                    {#each row.values as value, columnIndex}
                      <td
                        class:table-active={frame.table.active.some(
                          ([activeRow, activeColumn]) =>
                            activeRow === rowIndex && activeColumn === columnIndex
                        )}
                        class="rounded border border-bg-border bg-bg-card px-3 py-2 font-mono font-semibold text-ink"
                      >{value}</td>
                    {/each}
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <aside class="border-t border-bg-border bg-bg-soft/45 p-5 lg:border-l lg:border-t-0">
        <div class="text-[11px] font-semibold uppercase tracking-wider text-ink-dim">当前规则</div>
        <code class="mt-2 block rounded-lg border border-bg-border bg-bg-card p-3 text-xs leading-5 text-accent">
          {frame.codeLine}
        </code>
        {#if frame.stack?.length}
          <div class="mt-5 text-[11px] font-semibold uppercase tracking-wider text-ink-dim">状态 / 调用栈</div>
          <div class="mt-2 space-y-1.5">
            {#each frame.stack as item}
              <div class="rounded border border-bg-border bg-bg-card px-3 py-2 font-mono text-xs text-ink-mute">
                {item}
              </div>
            {/each}
          </div>
        {/if}
        {#if frame.takeaway}
          <div class="mt-5 rounded-lg border border-accent/25 bg-accent/10 p-3 text-sm leading-6 text-ink">
            {frame.takeaway}
          </div>
        {/if}
      </aside>
    </div>

    <footer class="border-t border-bg-border px-4 py-3 sm:px-5">
      <div class="mb-3 h-1.5 overflow-hidden rounded-full bg-bg-soft">
        <div
          class="h-full rounded-full bg-accent transition-[width] duration-300"
          style:width={`${((frameIndex + 1) / spec.frames.length) * 100}%`}
        ></div>
      </div>
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div class="flex items-center gap-1">
          <button class="btn-ghost" on:click={reset} aria-label="回到第一步"><RotateCcw size={15} /></button>
          <button class="btn-ghost" on:click={previous} disabled={frameIndex === 0} aria-label="上一步">
            <ChevronLeft size={17} /> 上一步
          </button>
          <button class="btn-primary" on:click={togglePlay} aria-label={playing ? '暂停动画' : '播放动画'}>
            {#if playing}<Pause size={16} /> 暂停{:else}<Play size={16} /> 播放{/if}
          </button>
          <button class="btn-ghost" on:click={next} disabled={frameIndex === lastIndex} aria-label="下一步">
            下一步 <ChevronRight size={17} />
          </button>
        </div>
        <label class="flex items-center gap-2 text-xs text-ink-mute">
          速度
          <select class="rounded border border-bg-border bg-bg-card px-2 py-1 text-ink" bind:value={speed} disabled={reduceMotion}>
            <option value={1700}>慢</option>
            <option value={1100}>正常</option>
            <option value={650}>快</option>
          </select>
          {#if reduceMotion}<span>已遵循减少动态效果设置</span>{/if}
        </label>
      </div>
    </footer>
  </section>
{:else}
  <div class="card p-5 text-sm text-ink-mute">该动态图解暂不可用。</div>
{/if}

<style>
  .visualizer:focus-visible {
    outline: 2px solid rgb(var(--c-accent));
    outline-offset: 3px;
  }
  .visual-cell {
    display: grid;
    width: 3.25rem;
    height: 3.25rem;
    place-items: center;
    border: 1px solid rgb(var(--c-bg-border));
    border-radius: 0.65rem;
    background: rgb(var(--c-bg-card));
    color: rgb(var(--c-ink));
    font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
    font-weight: 700;
    transition: 180ms ease;
  }
  .visual-cell.state-active,
  .visual-cell.state-pivot {
    border-color: rgb(var(--c-accent));
    background: rgb(var(--c-accent) / 0.16);
    color: rgb(var(--c-accent));
    transform: translateY(-0.25rem);
  }
  .visual-cell.state-candidate {
    border-color: rgb(var(--c-medium));
    background: rgb(var(--c-medium) / 0.12);
    color: rgb(var(--c-medium));
  }
  .visual-cell.state-done {
    border-color: rgb(var(--c-easy) / 0.55);
    background: rgb(var(--c-easy) / 0.12);
    color: rgb(var(--c-easy));
  }
  .visual-cell.state-muted {
    opacity: 0.35;
  }
  .visual-edge {
    stroke: rgb(var(--c-bg-border));
    stroke-width: 1.2;
  }
  .visual-edge.state-active,
  .visual-edge.state-candidate {
    stroke: rgb(var(--c-accent));
    stroke-width: 2;
  }
  .visual-edge.state-done {
    stroke: rgb(var(--c-easy));
    stroke-width: 1.8;
  }
  .visual-edge.state-muted {
    opacity: 0.25;
  }
  .visual-node circle {
    fill: rgb(var(--c-bg-card));
    stroke: rgb(var(--c-bg-border));
    stroke-width: 1;
    transition: 180ms ease;
  }
  .visual-node text {
    fill: rgb(var(--c-ink));
    font-size: 4px;
    font-weight: 700;
  }
  .visual-node.state-active circle,
  .visual-node.state-candidate circle {
    fill: rgb(var(--c-accent) / 0.16);
    stroke: rgb(var(--c-accent));
    stroke-width: 1.8;
  }
  .visual-node.state-done circle {
    fill: rgb(var(--c-easy) / 0.12);
    stroke: rgb(var(--c-easy));
  }
  .visual-node.state-muted {
    opacity: 0.3;
  }
  .edge-label {
    fill: rgb(var(--c-ink-mute));
    font-size: 3.5px;
    text-anchor: middle;
  }
  .table-active {
    border-color: rgb(var(--c-accent));
    background: rgb(var(--c-accent) / 0.14);
    color: rgb(var(--c-accent));
  }
  button:disabled {
    cursor: not-allowed;
    opacity: 0.4;
  }
  @media (prefers-reduced-motion: reduce) {
    .visual-cell,
    .visual-node circle {
      transition: none;
    }
  }
</style>
