<!-- H5.vue -->
<template>
  <h5 :id="id" class="text-h4 font-bold mt-5 mb-3">
    <slot />
  </h5>
</template>
<script setup lang="ts">
defineProps<{ id?: string }>()
</script>