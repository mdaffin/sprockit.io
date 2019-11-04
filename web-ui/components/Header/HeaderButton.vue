<template>
  <button @click="$emit('click')">
    <slot />
  </button>
</template>

<style scoped>
button {
  background-color: var(--background-color);
  color: var(--color);
  cursor: pointer;
  margin: 0;
  height: 100%;
  border: none;
}

button.primary {
  background-color: var(--green);
  color: var(--dark-green);
}
</style>
