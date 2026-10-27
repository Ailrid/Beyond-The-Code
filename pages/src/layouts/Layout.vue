<template>
  <div class="w-full">
    <router-view v-slot="{ Component }">
      <component :is="Component" />
    </router-view>
  </div>
</template>
<script setup lang="ts">
  import { RouterView } from "vue-router";
</script>

<style lang="css"></style>
