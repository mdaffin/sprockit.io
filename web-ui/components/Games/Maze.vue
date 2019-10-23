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
        empty: "░░",
        exit: "▒▒",
        hidden: "??",
        open: "  ",
        player: "⋐⋑",
      },
    };
  },
  methods: {
    emptyMap() {
      return Array.from({ length: 10 }, () =>
        Array.from({ length: 10 }, () => "empty"),
      );
    },
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
  border: solid 4px #373737;
  font-size: 2.25vw;
}
</style>
