<!-- code/Fun.vue -->
<template>
  <span class="text-code-function">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>