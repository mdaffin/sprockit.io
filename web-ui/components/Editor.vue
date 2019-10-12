<template>
  <div>
    <div class="editor-panel-handle">
      <div
        class="editor-panel-handle-tab"
        :class="{ 'is-selected': tab === 'Script' }"
      >
        <span @click="handleTab('Script')">Script</span>
      </div>
      <div
        class="editor-panel-handle-tab"
        :class="{ 'is-selected': tab === 'Console' }"
      >
        <span @click="handleTab('Console')">Console</span>
      </div>
      <ExecuteButton @click="$emit('run')" />
    </div>
    <client-only placeholder="Codemirror Loading...">
      <codemirror
        :value="value"
        @input="$emit('input', $event)"
        :options="cmOption"
      />
    </client-only>
  </div>
</template>

<script>
import ExecuteButton from "~/components/EditorComponents/ExecuteButton";

export default {
  components: {
    ExecuteButton,
  },
  props: ["value"],
  data() {
    return {
      tab: "Script",
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
  methods: {
    handleTab(tab) {
      this.tab = tab;
    },
  },
};
</script>

<style>
.CodeMirror,
.vue-codemirror {
  height: 100%;
  width: 100%;
}
</style>

<style scoped>
.editor-panel-handle {
  position: relative;
  right: 35px;
  margin-right: -35px;
  background-color: #2a3a4b;
  cursor: e-resize;
}

.editor-panel-handle-tab {
  width: 35px;
  height: 100px;
  background: #2979d3;
  color: white;
  display: flex;
  cursor: pointer;
}

.editor-panel-handle-tab span {
  height: 35px;
  min-width: 100px;
  padding: 5px;
  text-align: center;
  transform-origin: 0 0;
  transform: rotate(-90deg) translate(-100px, 0);
}

.is-selected {
  background: purple;
}
</style>
