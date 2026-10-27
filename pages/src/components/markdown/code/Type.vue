<!-- code/Type.vue -->
<template>
  <span class="text-code-type">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>