<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { navigating } from '$app/stores';
  import { afterNavigate } from '$app/navigation';
  import { Menu, Sparkles } from 'lucide-svelte';

  let drawerOpen = false;
  // 路由切换完成后自动收起移动端抽屉。
  afterNavigate(() => {
    drawerOpen = false;
  });
</script>

<!-- 全局导航加载条 -->
{#if $navigating}
  <div class="nav-loading-bar"></div>
{/if}

<!-- 移动端顶栏（≥md 隐藏） -->
<header
  class="sticky top-0 z-30 flex items-center gap-3 border-b border-bg-border bg-bg-soft/80 px-4 py-3 backdrop-blur md:hidden"
>
  <button
    on:click={() => (drawerOpen = true)}
    class="text-ink-mute transition hover:text-ink"
    aria-label="打开菜单"
  >
    <Menu size={20} />
  </button>
  <div class="flex items-center gap-2">
    <div class="grid h-6 w-6 place-items-center rounded bg-accent/15 text-accent ring-1 ring-accent/30">
      <Sparkles size={13} />
    </div>
    <span class="font-semibold tracking-tight text-ink">Algo Station</span>
  </div>
</header>

<div class="flex min-h-screen">
  <!-- 桌面侧栏 -->
  <div class="hidden md:block">
    <Sidebar />
  </div>

  <!-- 移动端抽屉 -->
  {#if drawerOpen}
    <button
      class="fixed inset-0 z-40 bg-black/50 md:hidden"
      aria-label="关闭菜单"
      on:click={() => (drawerOpen = false)}
    ></button>
    <div class="fixed inset-y-0 left-0 z-50 shadow-xl md:hidden">
      <Sidebar on:navigate={() => (drawerOpen = false)} />
    </div>
  {/if}

  <main class="min-w-0 flex-1 overflow-x-hidden">
    <slot />
  </main>
</div>

<style>
  .nav-loading-bar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    z-index: 60;
    background: rgb(var(--c-accent));
    animation: nav-loading 1s ease-in-out infinite;
    transform-origin: left;
  }
  @keyframes nav-loading {
    0% {
      transform: scaleX(0);
      opacity: 1;
    }
    50% {
      transform: scaleX(0.7);
      opacity: 1;
    }
    100% {
      transform: scaleX(1);
      opacity: 0.4;
    }
  }
</style>
