<template>
  <div class="output" id="console" v-html="computedConsole" />
</template>

<script>
export default {
  props: {
    console: { type: Array, default: [] },
  },
  computed: {
    computedConsole() {
      let computedConsole = "";
      this.console.forEach(line => {
        computedConsole += `<div class = "console-line ${line.type}">${line.output}</div>`;
      });
      return computedConsole;
    },
  },
};
</script>

<style>
.console-line {
  padding: 5px;
}
.console-line:nth-child(2n) {
  background-color: #e6dfd7;
}
.error {
  color: red;
}
.norm {
  color: #2b2b2b;
}
</style>
