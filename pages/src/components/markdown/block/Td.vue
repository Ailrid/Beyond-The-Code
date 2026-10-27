<!-- Td.vue -->
<template>
  <td
    :style="{ textAlign: align }"
    class="border border-article-table-border px-3 py-1.5"
  >
    <slot />
  </td>
</template>
<script setup lang="ts">
  defineProps<{ align?: "left" | "center" | "right" }>();
</script>
