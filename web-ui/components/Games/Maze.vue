<template>
  <div v-if="!hasRun" class="maze-game-ascii">
    <pre>{{ this.emptyMap }}</pre>
  </div>
  <div v-else>
    <div class="maze-game-ascii">
      <pre>{{ this.gameState.visualMap }}</pre>
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
      gameState: {
        visualMap: this.emptyMaze(),
      },
      emptyMap: this.emptyMaze(),
    };
  },
  props: {
    value: { type: String, default: "" },
  },
  updated() {
    this.getMaze();
  },
  methods: {
    async getMaze() {
      const data = (await this.fetchMaze()).data;
      const map = data.map;
      const [playerX, playerY] = [data.player.x, data.player.y];
      const [exitX, exitY] = [data.exit.x, data.exit.y];

      map[playerY][playerX] = "player";
      map[exitY][exitX] = "exit";

      this.gameState.visualMap = map
        .map(x => x.map(y => this.gameViz[y]).join(""))
        .join("\n");
    },
    async fetchMaze() {
      return await this.$axios.get("/api/game/maze/map", {
        headers: { "X-TOKEN": this.$store.state.token },
      });
    },
    emptyMaze() {
      return Array.from({ length: 10 }, () => " ".repeat(20)).join("\n");
    },
  },
  computed: {
    hasRun() {
      return this.$store.state.run;
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
