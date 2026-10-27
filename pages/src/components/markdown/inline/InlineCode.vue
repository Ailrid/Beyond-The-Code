<!-- inline/InlineCode.vue -->
<template>
  <code class="font-mono text-[0.9em] font-semibold
               bg-article-code-bg text-article-code-fg
               rounded px-1 py-0.5">{{ text }}</code>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>