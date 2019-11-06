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
  data() {
    return {
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
  },
};
</script>

<style scoped>
.editor-panel {
  display: flex;
  width: var(--output-width);
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
}

.editor-panel-header-tab {
  width: 35px;
  height: 100px;
  background: #dcd0c0;
  display: flex;
  cursor: pointer;
  color: var(--background-color);
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
</style>
