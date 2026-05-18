<script lang="ts">
  export let difficulty: 'Easy' | 'Medium' | 'Hard';
  const map = {
    Easy: { cls: 'chip-easy', label: '简单' },
    Medium: { cls: 'chip-medium', label: '中等' },
    Hard: { cls: 'chip-hard', label: '困难' }
  } as const;
</script>

<span class="chip {map[difficulty].cls}">{map[difficulty].label}</span>
