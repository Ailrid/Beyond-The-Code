<!-- code/String.vue -->
<template>
  <span class="text-code-string">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>