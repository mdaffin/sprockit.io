<template>
  <client-only placeholder="Codemirror Loading...">
    <codemirror v-model="script" :options="cmOption" />
  </client-only>
</template>

<script>
export default {
  computed: {
    script: {
      get() {
        return this.$store.state.script;
      },
      set(value) {
        this.$store.commit("setScript", value);
      },
    },
  },
  mounted() {
    const script = localStorage.mazeScript;
    if (script) {
      this.$store.commit("setScript", script);
    }
  },
  data() {
    return {
      cmOption: {
        tabSize: 4,
        styleActiveLine: true,
        lineNumbers: true,
        line: true,
        foldGutter: true,
        styleSelectedText: true,
        mode: "text/javascript",
        keyMap: "sublime",
        matchBrackets: true,
        showCursorWhenSelecting: true,
        theme: "sprockit",
        extraKeys: { Ctrl: "autocomplete" },
        hintOptions: {
          completeSingle: false,
        },
      },
    };
  },
};
</script>

<style>
.CodeMirror {
  height: 100%;
}
</style>
