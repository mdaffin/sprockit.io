<template>
  <div class="app">
    <Menu class="header" />
    <GamePanel class="game-panel" />
    <EditorPanel class="editor-panel" :console="console" />
  </div>
</template>

<script>
import Menu from "~/components/Menu.vue";
import EditorPanel from "~/components/EditorPanel.vue";
import GamePanel from "~/components/GamePanel.vue";

export default {
  components: {
    Menu,
    EditorPanel,
    GamePanel,
  },
  data() {
    return {
      console: [],
    };
  },
};
</script>

<style scoped>
:root {
  --output-width: 30%;
}

.app {
  display: grid;
  height: 100vh;
  grid-template-columns: 1fr var(--output-width);
  grid-template-rows: 42px 1fr;
  grid-template-areas:
    "header header"
    "game editor";
}

.editor-panel {
  grid-area: editor;
}

.game-panel {
  grid-area: game;
}

.header {
  grid-area: header;
}
</style>
