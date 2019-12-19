<template>
  <div class="left-hand-side">
    <div class="game-frame">
      <Maze class="game" />
    </div>
    <div class="help-frame">
      <HelpPanel />
    </div>
  </div>
</template>

<script>
import Maze from "~/components/Games/Maze.vue";
import HelpPanel from "~/components/HelpPanel.vue";

export default {
  components: {
    Maze,
    HelpPanel,
  },
};
</script>

<style scoped>
.game {
  margin: 0;
  padding: 2em;
  color: var(--color);
}
</style>
