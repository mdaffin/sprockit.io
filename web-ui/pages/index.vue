<template>
  <div class="app">
    <Header />
    <main class="panel">
      <GamePanel class="game-panel" />
      <DragHandle @drag="mainDragHandle" />
      <div class="editor-panel panel vertical">
        <Editor class="panel-editor" />
        <DragHandle @drag="editorDragHandle" />
        <Console class="panel-console" />
      </div>
    </main>
  </div>
</template>

<script>
import Console from "~/components/EditorPanelComponents/Console";
import DragHandle from "~/components/DragHandle.vue";
import Editor from "~/components/EditorPanelComponents/Editor";
import GamePanel from "~/components/GamePanel.vue";
import Header from "~/components/Header/Header.vue";

export default {
  components: {
    Header,
    DragHandle,
    Editor,
    Console,
    GamePanel,
  },
  mounted() {
    if (typeof Storage !== "undefined") {
      document.documentElement.style.setProperty(
        "--game-panel-width",
        localStorage.gamePanelWidth || "50%",
      );
    }
    if (typeof Storage !== "undefined") {
      document.documentElement.style.setProperty(
        "--editor-panel-width",
        localStorage.gamePanelWidth || "60%",
      );
    }
  },
  methods: {
    mainDragHandle(e) {
      const outputWidth = `${100 - Math.min(100, Math.max(0, e.percentageX))}%`;
      document.documentElement.style.setProperty(
        "--game-panel-width",
        outputWidth,
      );
      if (typeof Storage !== "undefined") {
        localStorage.gamePanelWidth = window
          .getComputedStyle(document.documentElement)
          .getPropertyValue("--game-panel-width");
      }
    },
    editorDragHandle(e) {
      const outputHeight = `${100 -
        Math.min(100, Math.max(0, e.percentageY))}%`;
      document.documentElement.style.setProperty(
        "--editor-panel-width",
        outputHeight,
      );
      if (typeof Storage !== "undefined") {
        localStorage.gamePanelWidth = window
          .getComputedStyle(document.documentElement)
          .getPropertyValue("--editor-panel-width");
      }
    },
  },
};
</script>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  flex-direction: column;
}

.panel {
  height: 100%;
  display: flex;
  overflow: auto;
}

.panel.vertical {
  flex-direction: column;
}

.game-panel {
  width: var(--game-panel-width);
  overflow: auto;
}

.panel > .editor-panel {
  width: calc(100% - var(--game-panel-width));
  overflow: auto;
}

.vue-codemirror {
  flex-grow: 1;
}

.panel-editor {
  height: var(--editor-panel-width);
  overflow: auto;
}
.panel-console {
  height: calc(100% - var(--editor-panel-width));
  overflow: auto;
}
</style>
