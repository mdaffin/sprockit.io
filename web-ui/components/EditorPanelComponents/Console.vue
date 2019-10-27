<template>
  <div class="console">
    <div
      class="console-line"
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
  padding: 0.3em;
}
.console-line {
  color: #2b2b2b;
}
.error {
  color: red;
}
</style>
