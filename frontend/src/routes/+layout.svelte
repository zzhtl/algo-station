<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
</script>

<div class="flex min-h-screen">
  <Sidebar />
  <main class="min-w-0 flex-1 overflow-x-hidden">
    <slot />
  </main>
</div>
