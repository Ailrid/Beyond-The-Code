<!-- Ol.vue -->
<template>
  <ol :start="start" class="list-decimal pl-8 my-[0.8em]">
    <slot />
  </ol>
</template>
<script setup lang="ts">
defineProps<{ start?: number }>()
</script>
