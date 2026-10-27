<!-- code/Plain.vue -->
<template>
  <span class="text-code-plain">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>