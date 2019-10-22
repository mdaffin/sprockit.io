<template>
  <div v-if="!hasRun" class="maze-game-ascii">
    <pre>{{ this.map }}</pre>
  </div>
  <div v-else>
    <div class="maze-game-ascii">
      <pre>{{ this.map }}</pre>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      gameViz: {
        blocked: "██",
        exit: "▒▒",
        hidden: "??",
        open: "  ",
        player: "⋐⋑",
      },
      map: this.emptyMaze(),
    };
  },
  props: {
    value: { type: String, default: "" },
  },
  updated() {
    this.drawMaze();
  },
  methods: {
    async drawMaze() {
      const data = await this.$store.dispatch("FETCH_MAZE");
      const map = data.map;
      const [playerX, playerY] = [data.player.x, data.player.y];
      const [exitX, exitY] = [data.exit.x, data.exit.y];

      map[playerY][playerX] = "player";
      map[exitY][exitX] = "exit";

      this.map = map.map(x => x.map(y => this.gameViz[y]).join("")).join("\n");
    },
    emptyMaze() {
      return Array.from({ length: 10 }, () => "░▓".repeat(10)).join("\n");
    },
  },
  computed: {
    hasRun() {
      return this.$store.state.run;
    },
    mazeState() {
      return this.$store.state.maze;
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
