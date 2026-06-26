<script lang="ts">
  import { invalidateAll } from '$app/navigation';
  import { TriangleAlert, RotateCcw } from 'lucide-svelte';

  export let message = '加载失败，请确认后端服务是否在运行。';
  let retrying = false;

  async function retry() {
    retrying = true;
    await invalidateAll();
    retrying = false;
  }
</script>

<div class="card flex flex-col items-center gap-3 p-10 text-center">
  <div class="grid h-12 w-12 place-items-center rounded-full bg-hard/10 text-hard ring-1 ring-hard/20">
    <TriangleAlert size={22} />
  </div>
  <div class="max-w-sm text-sm text-ink-mute">{message}</div>
  <button class="btn-primary text-sm" on:click={retry} disabled={retrying}>
    <RotateCcw size={14} /> {retrying ? '重试中…' : '重试'}
  </button>
</div>
