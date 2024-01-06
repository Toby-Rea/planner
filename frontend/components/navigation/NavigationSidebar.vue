<script setup lang="ts">
const links = ref([
  {
    href: '/',
    icon: 'tabler:home',
  },
  {
    href: '/goals',
    icon: 'tabler:checkbox',
  },
  {
    href: '/budget',
    icon: 'tabler:wallet',
  },
  {
    href: '/habits',
    icon: 'tabler:calendar-bolt',
  },
  {
    href: '/settings',
    icon: 'tabler:settings',
  },
]);
</script>

<template>
  <aside class="bg-primary/20 shadow-[inset_0px_0_6px_0px_rgba(0,0,0,0.2)] h-screen justify-center sticky left-0 top-0">
    <div class="grid px-3 py-6">
      <h1 class="text-3xl font-bold text-center pointer-events-none drop-shadow-lg">
        P.
      </h1>
      <nav class="grid gap-4 mt-16">
        <template v-for="link in links" :key="link.href">
          <NavigationLink :href="link.href" :icon="link.icon" />
        </template>
      </nav>
    </div>
  </aside>
</template>
