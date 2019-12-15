<template>
  <div class="maze-game-ascii">
    <pre>{{ this.drawMaze }}</pre>
  </div>
</template>

<script>
export default {
  data() {
    return {
      gameViz: {
        blocked: "██",
        hidden: "░░",
        exit: "▒▒",
        open: "  ",
        player: "⋐⋑",
      },
    };
  },
  mounted() {
    window.updateMaze = maze => {
      this.$store.commit("setMaze", maze);
    };
  },
  computed: {
    drawMaze() {
      return this.$store.state.maze
        .map(x => x.map(y => this.gameViz[y]).join(""))
        .join("\n");
    },
  },
};
</script>

<style scoped>
.maze-game-ascii {
  display: flex;
  align-items: center;
  justify-content: center;
}

.maze-game-ascii > pre {
  border: solid 0.2em var(--color);
  font-size: 2.25vw;
}
</style>
