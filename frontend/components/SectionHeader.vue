<script setup lang="ts">
const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  subtitle: {
    type: String,
    required: false,
    default: '',
  },
});
</script>

<template>
  <header class="bg-primary/5 border-l-[3px] border-primary rounded-lg px-8 py-6">
    <h1 class="text-3xl font-bold tracking-tight">
      {{ props.title }}
    </h1>
    <p
      v-if="props.subtitle"
      class="mt-2 text-sm text-stone-700"
    >
      {{ props.subtitle }}
    </p>
  </header>
</template>
