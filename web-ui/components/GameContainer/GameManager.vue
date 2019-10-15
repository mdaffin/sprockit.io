<template>
  <component :is="game" />
</template>

<script>
import Maze from "~/components/GameContainer/Maze/Maze.vue";

export default {
  components: {
    Maze,
  },
  props: {
    game: { type: String, default: "" },
  },
};
</script>
