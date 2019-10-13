<template>
  <div>
    <div class="editor-panel-header">
      <div
        class="editor-panel-header-tab"
        :class="{ 'is-selected': currentTabComponent === 'Editor' }"
      >
        <span @click="handleTab('Editor')">Editor</span>
      </div>
      <div
        class="editor-panel-header-tab"
        :class="{ 'is-selected': currentTabComponent === 'Console' }"
      >
        <span @click="handleTab('Console')">Console</span>
      </div>
      <ExecuteButton @click="$emit('run')" />
      <div id="editor-panel-header-handle" @mousedown="startDrag" />
    </div>
    <Editor
      class="editor"
      :class="{ 'block-highlight': isResizing }"
      @input="$emit('input', $event)"
      v-model="value"
      v-show="currentTabComponent === 'Editor'"
    />
    <Console
      :class="{ 'block-highlight': isResizing }"
      v-show="currentTabComponent === 'Console'"
    />
  </div>
</template>

<script>
import ExecuteButton from "~/components/EditorPanelComponents/ExecuteButton";
import Editor from "~/components/EditorPanelComponents/Editor";
import Console from "~/components/EditorPanelComponents/Console";

export default {
  components: {
    ExecuteButton,
    Editor,
    Console,
  },
  props: {
    value: { type: String, default: "" },
  },
  data() {
    return {
      isResizing: false,
      currentTabComponent: "Editor",
    };
  },
  methods: {
    handleTab(tab) {
      this.currentTabComponent = tab;
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
      let windowWidth =
        window.innerWidth ||
        document.documentElement.clientWidth ||
        document.body.clientWidth;
      let width = windowWidth - e.clientX;

      if (width > window.innerWidth) {
        width = window.innerWidth;
      } else if (width < 0) {
        width = 0;
      }
      if (this.isResizing)
        document.documentElement.style.setProperty(
          "--output-width",
          `${width / ((width + e.clientX) / 100)}%`,
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
.editor {
  background: #151515;
  width: 100%;
}

.editor-panel-header {
  position: relative;
  right: 35px;
  margin-right: -35px;
  cursor: e-resize;
}

.editor-panel-header-tab {
  width: 35px;
  height: 100px;
  background: #2979d3;
  color: white;
  display: flex;
  cursor: pointer;
}

.editor-panel-header-tab span {
  height: 35px;
  min-width: 100px;
  padding: 5px;
  text-align: center;
  transform-origin: 0 0;
  transform: rotate(-90deg) translate(-100px, 0);
}

#editor-panel-header-handle {
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
