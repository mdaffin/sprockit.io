<template>
  <div class="game-frame">
    <Maze class="game" />
  </div>
</template>

<script>
import Maze from "~/components/Games/Maze.vue";

export default {
  components: {
    Maze,
  },
};
</script>

<style scoped>
.game {
  margin: 2em;
  padding: 2em;
  border-radius: 2em;
  background-color: #c0b283;
}
</style>
