<template>
  <div>
    <div class="editor-panel">
      <div
        class="editor-panel-tab"
        :class="{ 'is-selected': tab === 'Script' }"
      >
        <span @click="handleTab('Script')">Script</span>
      </div>
      <div
        class="editor-panel-tab"
        :class="{ 'is-selected': tab === 'Console' }"
      >
        <span @click="handleTab('Console')">Console</span>
      </div>
      <ExecuteButton @click="$emit('run')" />
      <div id="editor-panel-handle" @mousedown="startDrag" />
    </div>
    <client-only placeholder="Codemirror Loading...">
      <codemirror
        v-show="tab === 'Script'"
        :class="{ 'block-highlight': isResizing }"
        :value="value"
        @input="$emit('input', $event)"
        :options="cmOption"
      />
      <div class="output" id="output-area" v-show="tab === 'Console'">
        <iframe id="output" />
      </div>
    </client-only>
  </div>
</template>

<script>
import ExecuteButton from "~/components/EditorComponents/ExecuteButton";

export default {
  components: {
    ExecuteButton,
  },
  props: {
    value: { type: String, default: "" },
  },
  data() {
    return {
      tab: "Script",
      isResizing: false,
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
    startDrag() {
      this.isResizing = true;
      window.addEventListener("mousemove", e => this.resizeEditor(e));
      window.addEventListener("mouseup", this.stopDrag);
    },
    stopDrag() {
      window.removeEventListener("mousemove", e => this.resizeEditor(e));
      window.removeEventListener("mouseup", this.stopDrag);
      this.isResizing = false;
    },
    resizeEditor(e) {
      let width = window.innerWidth - e.clientX;
      let windowWidth =
        window.innerWidth ||
        document.documentElement.clientWidth ||
        document.body.clientWidth;

      if (width > window.innerWidth) {
        width = window.innerWidth;
      } else if (width < 0) {
        width = 0;
      }
      if (this.isResizing)
        document.documentElement.style.setProperty(
          "--output-width",
          `${width / (windowWidth / 100)}%`,
        );
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
.editor-panel {
  position: relative;
  right: 35px;
  margin-right: -35px;
  cursor: e-resize;
}

.editor-panel-tab {
  width: 35px;
  height: 100px;
  background: #2979d3;
  color: white;
  display: flex;
  cursor: pointer;
}

.editor-panel-tab span {
  height: 35px;
  min-width: 100px;
  padding: 5px;
  text-align: center;
  transform-origin: 0 0;
  transform: rotate(-90deg) translate(-100px, 0);
}

#editor-panel-handle {
  height: calc(100% - 235px);
  background-color: #2a3a4b;
}

.is-selected {
  background: purple;
}

.is-resizing {
  background: #304155;
}

.block-highlight {
  user-select: none;
}
</style>
