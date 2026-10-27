<!-- CodeBlock.vue -->
<template>
  <div class="code-block my-5 rounded bg-article-code-bg">
    <div
      class="flex items-center justify-between px-3 py-1.5 border-b border-article-rule text-sm text-muted-foreground"
    >
      <span class="font-mono">{{ lang }}</span>
      <button class="hover:text-foreground transition" @click="copy">
        {{ copied ? "已复制" : "复制" }}
      </button>
    </div>
    <pre
      ref="preEl"
      class="overflow-x-auto p-4 font-mono text-sm leading-relaxed"
    ><slot /></pre>
  </div>
</template>
<script setup lang="ts">
  import { ref } from "vue";
  defineProps<{ lang?: string }>();
  const preEl = ref<HTMLElement>();
  const copied = ref(false);
  async function copy() {
    await navigator.clipboard.writeText(preEl.value?.textContent ?? "");
    copied.value = true;
    setTimeout(() => (copied.value = false), 1500);
  }
</script>
