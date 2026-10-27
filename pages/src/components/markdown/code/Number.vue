<!-- code/Number.vue -->
<template>
  <span class="text-code-number">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>