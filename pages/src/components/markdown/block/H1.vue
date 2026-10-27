<!-- H1.vue -->
<template>
  <h1
    :id="id"
    class="text-h1 font-bold text-center pb-[0.1em] my-6 border-b border-article-rule"
  >
    <slot />
  </h1>
</template>
<script setup lang="ts">
  defineProps<{ id?: string }>();
</script>
