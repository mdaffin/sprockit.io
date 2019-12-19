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

<script>
export default {
  updated: function() {
    this.$nextTick(function() {
      const panel = document.querySelector(".panel-console");
      panel.scrollBy(0, panel.scrollHeight);
    });
  },
};
</script>

<style>
.console {
  color: var(--color);
  background-color: var(--background-color);
}
.error {
  color: red;
}
</style>
