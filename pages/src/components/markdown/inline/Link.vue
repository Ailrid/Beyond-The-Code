<!-- inline/Link.vue -->
<template>
  <a
    :href="href"
    :target="isExternal ? '_blank' : undefined"
    :rel="isExternal ? 'noopener noreferrer' : undefined"
    class="text-article-link underline-offset-4 hover:underline
           transition-colors"
  >
    <slot />
  </a>
</template>
<script setup lang="ts">
import { computed } from 'vue'
const props = defineProps<{ href: string }>()
const isExternal = computed(() => /^https?:\/\//.test(props.href))
</script>