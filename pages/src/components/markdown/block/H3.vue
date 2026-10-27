<!-- H3.vue -->
<template>
  <h3
    :id="id"
    class="text-h3 font-bold border-l-2 border-foreground pl-2.5 mt-6 mb-4"
  >
    <slot />
  </h3>
</template>
<script setup lang="ts">
  defineProps<{ id?: string }>();
</script>
