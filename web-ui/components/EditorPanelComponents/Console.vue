<template>
  <div class="console">
    <div
      :class="[line.type]"
      v-for="(line, index) in $store.state.console.lines"
      :key="index"
    >
      {{ line.output }}
    </div>
  </div>
</template>

<style>
.console {
  color: var(--background-color);
  background-color: var(--color);
}
.error {
  color: red;
}
</style>
