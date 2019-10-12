<template>
  <client-only placeholder="Codemirror Loading...">
    <codemirror v-model="code" :options="cmOption" />
  </client-only>
</template>

<script>
export default {
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
        theme: "base16-dark",
        extraKeys: { Ctrl: "autocomplete" },
        hintOptions: {
          completeSingle: false,
        },
      },
    };
  },
  props: {
    code: {
      type: String,
      required: true,
    },
  },
};
</script>

<style>
.CodeMirror {
  height: 100%;
}
</style>
