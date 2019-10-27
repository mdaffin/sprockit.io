<template>
  <div class="output" id="console">
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
.output {
  padding: 0.3em;
}
.error {
  color: red;
}
.norm {
  color: #2b2b2b;
}
</style>
