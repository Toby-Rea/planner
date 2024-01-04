<script setup lang="ts">
useHead({
  title: 'Planner.',
});
</script>

<template>
  <main>
    <NavigationSidebar />
    <div id="content">
      <slot />
    </div>
  </main>
</template>

<style>
@import url("~/assets/css/main.css");

main {
  display: flex;
  flex-direction: row;
  width: 100vw;
}

#content {
  padding: 4rem;
  width: 100%;
}
</style>
