<template>
  <button @click="$emit('click')">
    <slot />
  </button>
</template>

<style scoped>
button {
  background: #6dc957;
  border: none;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
