<template>
  <div class="output" id="console" v-html="console" />
</template>

<script>
export default {
  props: {
    console: { type: String, default: "" },
  },
};
</script>
