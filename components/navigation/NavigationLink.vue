<script setup lang="ts">
const props = defineProps({
  href: {
    type: String,
    required: true,
  },
  icon: {
    type: String,
    required: true,
  },
});

const route = useRoute();
const isActive = computed(() => props.href === route.path);
</script>

<template>
  <NuxtLink :to="props.href" :class="{ 'link-active': isActive }">
    <Icon :name="props.icon" size="24" />
  </NuxtLink>
</template>

<style scoped>
a {
  all: unset;
  border-radius: 0.5rem;
  padding: 0.75rem;
  scale: 1.1;
  transition: all 150ms ease-in-out;
}

a:hover {
  background-color: rgba(28, 25, 23, 0.05);
  cursor: pointer;
}

.link-active {
  background-color: rgba(var(--primary), 0.2);
  box-shadow: inset 0px 0px 4px rgba(0, 0, 0, 0.1);
  color: rgba(var(--primary), 1);
}
</style>
