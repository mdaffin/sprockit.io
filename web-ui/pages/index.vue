<template>
  <div class="app">
    <Header />
    <main class="panel">
      <GamePanel class="game-panel" />
      <DragHandle @drag="mainDragHandle" />
      <div class="editor-panel panel vertical">
        <Editor />
        <DragHandle @drag="editorDragHandle" />
        <Console />
      </div>
    </main>
  </div>
</template>

<script>
import Header from "~/components/Header/Header.vue";
import DragHandle from "~/components/DragHandle.vue";
import GamePanel from "~/components/GamePanel.vue";
import Editor from "~/components/EditorPanelComponents/Editor";
import Console from "~/components/EditorPanelComponents/Console";

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
}

.panel.vertical {
  flex-direction: column;
}

.panel > .game-panel {
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

.console {
  height: 30%;
  overflow: auto;
}
</style>
