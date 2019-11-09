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
    document.addEventListener("keydown", e => {
      const modifierKey = navigator.platform.match("Mac")
        ? e.metaKey
        : e.ctrlKey;
      if ((e.key ? e.key === "s" : e.keyCode === 83) && modifierKey) {
        e.preventDefault();
        if (typeof Storage !== "undefined") {
          localStorage.mazeScript = this.$store.state.script;
        }
      }
    });
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
