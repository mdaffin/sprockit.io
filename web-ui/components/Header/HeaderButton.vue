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
  width: 5em;
  height: 100%;
  border: none;
}

button.primary {
  background-color: var(--green);
}

button:hover {
  color: white;
}

button.cyan:hover {
  background-color: var(--cyan);
}

button.purple:hover {
  background-color: var(--purple);
}

button.rose:hover {
  background-color: var(--rose);
}
</style>
