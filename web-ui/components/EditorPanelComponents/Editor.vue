<template>
  <client-only placeholder="Codemirror Loading...">
    <codemirror
      :value="value"
      @input="$emit('input', $event)"
      :options="cmOption"
    />
  </client-only>
</template>

<script>
export default {
  props: {
    value: { type: String, default: "" },
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
        theme: "duotone-light",
        extraKeys: { Ctrl: "autocomplete" },
        hintOptions: {
          completeSingle: false,
        },
      },
    };
  },
};
</script>
