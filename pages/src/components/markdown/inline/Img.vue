<!-- inline/Img.vue -->
<template>
  <img
    :src="src"
    :alt="alt"
    :title="title"
    class="max-w-full h-auto mx-auto my-4 rounded"
  />
</template>
<script setup lang="ts">
defineProps<{ src: string; alt?: string; title?: string }>()
</script>