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
      <div class = "editor-panel-handle"
        @mousedown="startDrag"
      />
    </div>
    <client-only placeholder="Codemirror Loading...">
      <codemirror 
        v-show="tab === 'Script'"
        :class="{'block-highlight' : isResizing }"
        :value="value"
        @input="$emit('input', $event)"
        :options="cmOption"
      />
      <div class="output" id="output-area"
        v-show="tab === 'Console'"
      >
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
    value: { 
      type: String, 
      default: "" 
    },
  },
  data() {
    return {
      tab: "Script",
      isResizing: false,
      percentInPx: 0,
      mouseCurrentX: null,
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
  mounted: function () {
    let windowWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
    this.percentInPx = windowWidth / 100;
    window.addEventListener("mousemove", (e) => this.resizeEditor(e));
    window.addEventListener("click", this.stopDrag);
    window.addEventListener('resize', () => this.updateWindowDimensions());
  },
  beforeDestroy() {
    window.removeEventListener("mousemove", (e) => this.resizeEditor(e));
    window.removeEventListener("click", this.stopDrag);
  },
  methods: {
    handleTab(tab) {
      this.tab = tab; 
    },
    updateWindowDimensions(e) {
      let windowWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
      this.percentInPx = windowWidth / 100;
    },
    startDrag() {
      this.isResizing = true;
    },
    stopDrag() {
      this.isResizing = false;
      document.documentElement.style.setProperty('--output-width', `${100 - this.mouseCurrentX/this.percentInPx}%`);
    },
    resizeEditor(e) {
      this.mouseCurrentX = e.clientX;
      let x = e.clientX;
      let diff = 0;
      if(!this.editorResizePx) {
        this.editorResizePx = x;
      } else {
        diff = x - this.editorResizePx;
        this.editorResizePx = x;
      }
      let currentEditorWidth = getComputedStyle(document.documentElement,null).getPropertyValue('--output-width');

      if(this.isResizing && currentEditorWidth.replace('%', '') - diff/this.percentInPx > 0) 
        document.documentElement.style.setProperty('--output-width', `${currentEditorWidth.replace('%', '') - diff/this.percentInPx}%`);
    }
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

.editor-panel-handle {
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
