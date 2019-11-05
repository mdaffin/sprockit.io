<template>
  <div class="app">
    <Header />
    <main>
      <GamePanel />
      <EditorPanel class="editor-panel" :console="console" />
    </main>
  </div>
</template>

<script>
import Header from "~/components/Header/Header.vue";
import EditorPanel from "~/components/EditorPanel.vue";
import GamePanel from "~/components/GamePanel.vue";

export default {
  components: {
    Header,
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
.app {
  display: flex;
  height: 100vh;
  flex-direction: column;
}

main {
  flex-grow: 1;
  display: flex;
}

main > * {
  flex-grow: 1;
}
</style>
