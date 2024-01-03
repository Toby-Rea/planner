<script setup lang="ts">
const links = ref([
  {
    href: '/',
    icon: 'tabler:home',
  },
  {
    href: '/goals',
    icon: 'tabler:calendar-check',
  },
  {
    href: '/budget',
    icon: 'tabler:wallet',
  },
  {
    href: '/settings',
    icon: 'tabler:settings',
  },
]);
</script>

<template>
  <aside>
    <div>
      <h1>P.</h1>
      <nav>
        <template v-for="link in links" :key="link.href">
          <NavigationLink :href="link.href" :icon="link.icon" />
        </template>
      </nav>
    </div>
  </aside>
</template>

<style scoped>
aside {
  background-color: rgba(var(--primary), 0.2);
  box-shadow: inset 0px 0 6px 0px rgba(0, 0, 0, 0.2);
  height: 100vh;
  justify-content: center;
  left: 0;
  position: sticky;
  top: 0;
}

aside > div {
  display: grid;
  padding: 1.5rem 0.75rem;
}

h1 {
  all: unset;
  filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.08)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));
  font-size: 30px;
  font-weight: 700;
  line-height: 36px;
  text-align: center;
}

nav {
  display: grid;
  gap: 1rem;
  margin-top: 4rem;
}
</style>
