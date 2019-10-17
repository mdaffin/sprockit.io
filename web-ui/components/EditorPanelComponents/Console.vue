<template>
  <div class="output" id="console" v-html="computedConsole" />
</template>

<script>
export default {
  props: {
    console: {
      type: Array,
      default() {
        return [];
      },
    },
  },
  computed: {
    computedConsole() {
      return this.console
        .map(
          line => `<div class="console-line ${line.type}">${line.output}</div>`,
        )
        .join();
    },
  },
};
</script>

<style>
.console-line {
  padding: 5px;
}
.error {
  color: red;
}
.norm {
  color: #2b2b2b;
}
</style>
