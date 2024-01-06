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
  <NuxtLink
    :to="props.href"
    :class="{ 'shadow-inner bg-primary/20 text-primary': isActive }"
    class="p-3 transition-all duration-150 ease-in-out scale-110 rounded-lg hover:bg-stone-700/5 hover:cursor-pointer"
  >
    <Icon :name="props.icon" size="24" />
  </NuxtLink>
</template>
