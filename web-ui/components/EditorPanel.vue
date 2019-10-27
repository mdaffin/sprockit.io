<template>
  <div class="editor-panel">
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
      <div id="editor-panel-header-handle" @mousedown="startDrag" />
    </div>
    <component
      class="panel-content"
      :is="currentTabComponent"
      :class="{ 'block-highlight': isResizing }"
      :[currentPropKey]="currentProp"
      @input="$emit('input', $event)"
    />
  </div>
</template>

<script>
import Editor from "~/components/EditorPanelComponents/Editor";
import Console from "~/components/EditorPanelComponents/Console";

export default {
  components: {
    Editor,
    Console,
  },
  props: {
    value: { type: String, default: "" },
    console: {
      type: Array,
      default() {
        return [];
      },
    },
  },
  mounted() {
    if (typeof Storage !== "undefined") {
      if (localStorage.editorWidth) {
        document.documentElement.style.setProperty(
          "--output-width",
          `${localStorage.editorWidth}`,
        );
      }
    }
  },
  data() {
    return {
      isResizing: false,
      currentTabComponent: "Editor",
    };
  },
  computed: {
    currentPropKey() {
      return this.currentTabComponent === "Editor" ? "value" : "console";
    },
    currentProp() {
      return this.currentTabComponent === "Editor" ? this.value : this.console;
    },
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
      if (typeof Storage !== "undefined")
        localStorage.editorWidth = window
          .getComputedStyle(document.documentElement)
          .getPropertyValue("--output-width");
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
      }

      if (width > 35) {
        width = `${width / ((width + e.clientX) / 100)}%`;
      } else {
        width = "35px";
      }

      if (this.isResizing)
        document.documentElement.style.setProperty("--output-width", width);
    },
  },
};
</script>

<style scoped>
.editor-panel {
  display: flex;
}

.CodeMirror,
.vue-codemirror {
  height: 100%;
  width: 100%;
}
</style>

<style scoped>
.panel-content {
  background: #faf8f5;
  width: 100%;
}

.editor-panel-header {
  position: relative;
  cursor: e-resize;
}

.editor-panel-header-tab {
  width: 35px;
  height: 100px;
  background: #dcd0c0;
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
  height: calc(100% - 200px);
  background-color: #f0f0f0;
}

.is-selected {
  background: #c0b283;
}

.is-resizing {
  background: #304155;
}

.block-highlight {
  user-select: none;
}
</style>
