<template>
  <div class="output" id="console">
    <div
      class="console-line"
      :class="[line.type]"
      v-for="(line, index) in this.console"
      :key="index"
    >
      {{ line.output }}
    </div>
  </div>
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
