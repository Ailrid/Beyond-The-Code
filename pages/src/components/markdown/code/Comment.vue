<!-- code/Comment.vue -->
<template>
  <span class="text-code-comment italic">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>