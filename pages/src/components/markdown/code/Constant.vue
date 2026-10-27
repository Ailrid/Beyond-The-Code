<!-- code/Constant.vue -->
<template>
  <span class="text-code-constant">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>