<template>
  <div class="container">
    <client-only placeholder="Codemirror Loading...">
      <vue-draggable-resizable
        id="editor"
        :active="true"
        @resizestop="updatePosition"
        :x="editorLeft"
        :w="editorComputedWidth"
        :h="pageHeight"
        :draggable="false"
        :handles="['ml']"
      >
        <div class="editor-panel">
          <div class="editor-panel-handle">
            <div
              class="editor-panel-handle-tab"
              v-bind:class="{ isSelected: tab === 'Script' }"
            >
              <span v-on:click="handleTab('Script')">Script</span>
            </div>
            <div
              class="editor-panel-handle-tab"
              v-bind:class="{ isSelected: tab === 'Console' }"
            >
              <span v-on:click="handleTab('Console')">Console</span>
            </div>
          </div>
          <codemirror v-model="code" :options="cmOption" />
        </div>
      </vue-draggable-resizable>
    </client-only>
  </div>
</template>

<script>
import VueDraggableResizable from "vue-draggable-resizable";

let width,
  height = 0;
if (process.browser) {
  width =
    window.innerWidth ||
    document.documentElement.clientWidth ||
    document.body.clientWidth;
  height =
    window.innerHeight ||
    document.documentElement.clientHeight ||
    document.body.clientHeight;
}

export default {
  data() {
    return {
      tab: "Script",
      code: "const A = 10",
      editorLeft: width,
      pageHeight: height,
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
          completeSingle: false
        }
      }
    };
  },
  computed: {
    editorComputedWidth: function() {
      return width - this.editorLeft;
    }
  },
  methods: {
    handleTab: function(event) {
      if (this.editorComputedWidth == 0) {
        this.editorLeft = width - 800;
      } else {
        this.tab = event;
      }
    },
    updatePosition: function(event) {
      this.editorLeft = event;
    }
  }
};
</script>

<style>
.editor-panel {
  display: flex;
  height: 100%;
}

.editor-panel-handle {
  position: relative;
  right: 35px;
  margin-right: -35px;
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
  transform-origin: 0 0;
  transform: rotate(-90deg) translate(-100px, 0);
}

.isSelected {
  background: #542fbf;
}

.container {
  margin: 0;
  min-height: 100vh;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.handle-ml,
.handle-mr {
  top: 200px;
  margin-top: 0;

  /*Had to do this because they use an inline style that toggles*/
  display: block !important;
  left: -35px;
  height: calc(100% - 200px);
  width: 35px;
  background: #2a3a4b;
  border: none;
  z-index: 10;
}
.vue-codemirror {
  height: 100%;
  width: 100%;
}
.CodeMirror {
  height: 100%;
  width: 100%;
  text-align: left;
}

.CodeMirror-sizer {
  width: 100%;
}

.cm-s-base16-dark .CodeMirror-gutters {
  background: #212d3b;
  border-right: 0px;
}

.cm-s-base16-dark .CodeMirror-linenumber {
  color: white;
  text-align: center;
}

.vdr {
  border: none;
}
</style>
