<!-- code/Punct.vue -->
<template>
  <span class="text-code-punct">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>