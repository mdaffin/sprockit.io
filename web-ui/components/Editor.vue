<template>
  <client-only placeholder="Codemirror Loading...">
    <div>
      <div class="editor-panel-handle">
        <div
          class="editor-panel-handle-tab"
          :class="{ isSelected: tab === 'Script' }"
        >
          <span @click="handleTab('Script')">Script</span>
        </div>
        <div
          class="editor-panel-handle-tab"
          :class="{ isSelected: tab === 'Console' }"
        >
          <span @click="handleTab('Console')">Console</span>
        </div>
        <ExecuteButton />
      </div>
      <codemirror v-model="code" :options="cmOption" @input="onCmCodeChange" />
    </div>
  </client-only>
</template>

<script>
import ExecuteButton from "~/components/EditorComponents/ExecuteButton";

export default {
  components: {
    ExecuteButton,
  },
  data() {
    return {
      code: "const a = 10;\nconst b = 20;\nconsole.log(a + b);\n",
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
  created() {
    if (this.$bus) this.$bus.emit("update-code", this.code);
  },
  methods: {
    onCmCodeChange(event) {
      this.$bus.emit("update-code", event);
    },
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

.isSelected {
  background: purple;
}
</style>
