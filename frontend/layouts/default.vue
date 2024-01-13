<script setup lang="ts">
useHead({
  title: 'Planner.',
});
</script>

<template>
  <main class="flex w-screen">
    <NavigationSidebar />
    <div class="w-full p-16">
      <slot />
    </div>
  </main>
</template>

<style>
@import url("~/assets/css/main.css");
</style>
