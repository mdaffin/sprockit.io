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
        Blocked: "██",
        Exit: "▒▒",
        Open: "  ",
        Player: "⋐⋑",
      },
      gameState: {
        visualMap: this.emptyMaze(),
      },
      emptyMap: this.emptyMaze(),
    };
  },
  updated() {
    this.getMaze();
  },
  methods: {
    async getMaze() {
      const data = (await this.fetchMaze()).data;
      const map = await data.map;
      const [playerX, playerY] = await [data.player.x, data.player.y];
      const [exitX, exitY] = await [data.exit.x, data.exit.y];

      map[playerY][playerX] = "Player";
      map[exitY][exitX] = "Exit";

      this.gameState.visualMap = await map
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
