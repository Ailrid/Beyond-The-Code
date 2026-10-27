<!-- H2.vue -->
<template>
  <h2 :id="id" class="text-h2 font-bold text-center w-[85%] mx-auto
                       py-2 my-5 border-b border-article-rule">
    <slot />
  </h2>
</template>
<script setup lang="ts">
defineProps<{ id?: string }>()
</script>