<!-- Th.vue -->
<template>
  <th
    :style="{ textAlign: align }"
    class="border border-article-table-border px-3 py-1.5 font-bold"
  >
    <slot />
  </th>
</template>
<script setup lang="ts">
  defineProps<{ align?: "left" | "center" | "right" }>();
</script>
