<script lang="ts">
  import { onMount } from 'svelte';
  import { Star } from 'lucide-svelte';
  import { bookmarks } from '$lib/stores/bookmarks';

  export let id: number;
  export let size = 16;

  onMount(() => bookmarks.load());
  $: active = $bookmarks.has(id);
</script>

<button
  type="button"
  class="inline-flex items-center justify-center rounded-md p-1.5 transition
    {active ? 'text-medium' : 'text-ink-dim hover:text-ink'}"
  title={active ? '取消收藏' : '收藏'}
  aria-label={active ? '取消收藏' : '收藏'}
  on:click|preventDefault|stopPropagation={() => bookmarks.toggle(id)}
>
  <Star {size} fill={active ? 'currentColor' : 'none'} />
</button>
