<!-- H4.vue -->
<template>
  <h4 :id="id" class="text-h4 font-bold mt-5 mb-3">
    <slot />
  </h4>
</template>
<script setup lang="ts">
defineProps<{ id?: string }>()
</script>