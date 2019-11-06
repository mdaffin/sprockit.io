<template>
  <div class="app">
    <Header />
    <main>
      <GamePanel class="game-panel" />
      <DragHandle @drag="dragHandle" />
      <EditorPanel class="editor-panel" :console="console" />
    </main>
  </div>
</template>

<script>
import Header from "~/components/Header/Header.vue";
import DragHandle from "~/components/DragHandle.vue";
import EditorPanel from "~/components/EditorPanel.vue";
import GamePanel from "~/components/GamePanel.vue";

export default {
  components: {
    Header,
    DragHandle,
    EditorPanel,
    GamePanel,
  },
  data() {
    return {
      console: [],
    };
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
  methods: {
    dragHandle(e) {
      const outputWidth = `${100 - Math.min(100, Math.max(0, e.percentageX))}%`;
      document.documentElement.style.setProperty("--output-width", outputWidth);
      if (typeof Storage !== "undefined") {
        localStorage.editorWidth = window
          .getComputedStyle(document.documentElement)
          .getPropertyValue("--output-width");
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

main {
  height: 100%;
  display: flex;
}

main > .game-panel {
  width: calc(100% - var(--output-width));
  overflow: auto;
}

main > .editor-panel {
  width: var(--output-width);
  overflow: auto;
}
</style>
