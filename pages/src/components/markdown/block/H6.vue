<!-- H6.vue -->
<template>
  <h6 :id="id" class="text-h4 font-bold mt-5 mb-3">
    <slot />
  </h6>
</template>
<script setup lang="ts">
defineProps<{ id?: string }>()
</script>