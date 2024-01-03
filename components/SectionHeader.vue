<script setup lang="ts">
const props = defineProps({
  title: {
    type: String,
    required: true
  },
  subtitle: {
    type: String,
    required: false,
    default: ''
  }
})
</script>

<template>
  <header>
    <h1>
      {{ props.title }}
    </h1>
    <p v-if="props.subtitle">
      {{ props.subtitle }}
    </p>
  </header>
</template>

<style scoped>
header {
  background-color: rgba(var(--primary), 0.05);
  border-left: 3px solid rgb(var(--primary));
  border-radius: 0.5rem;
  padding: 1.5rem 2rem 1.5rem 2rem;
}

header > h1 {
  all: unset;
  line-height: 36px;
  font-size: 30px;
  font-weight: 700;
  letter-spacing: -0.025em;
}

header > p {
  font-size: 14px;
  font-weight: 400;
  line-height: 20px;
  margin: 0.5rem 0 0 0;
}
</style>
