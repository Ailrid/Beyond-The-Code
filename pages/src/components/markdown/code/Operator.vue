<!-- code/Operator.vue -->
<template>
  <span class="text-code-operator">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>