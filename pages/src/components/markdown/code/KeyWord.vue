<template>
  <span class="text-code-keyword font-semibold">{{ text }}</span>
</template>
<script setup lang="ts">
defineProps<{ text: string }>()
</script>