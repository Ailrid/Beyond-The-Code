<!-- code/Variable.vue -->
<template>
  <span class="text-code-variable">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>