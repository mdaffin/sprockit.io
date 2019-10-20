<template>
  <div v-if="!hasRun" class="maze-game-ascii">
    <pre>{{ emptyMaze() }}</pre>
  </div>
  <div v-else>
    <div class="maze-game-ascii">
      <pre>{{ getMaze() }}</pre>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      gameState: null,
    };
  },
  methods: {
    async createGameSession() {
      this.gameViz = {
        Blocked: "██",
        Exit: "▒▒",
        Open: "  ",
        Player: "⋐⋑",
      };
      this.gameState = (await this.$axios.get("/api/game/maze/map", {
        headers: { "X-TOKEN": "5e017a67-2080-4a76-9f41-010cc1556e3a" },
      })).data;
    },
    getMaze() {
      this.createGameSession();

      const map = this.gameState.map;
      const [playerX, playerY] = [
        this.gameState.player.x,
        this.gameState.player.y,
      ];
      const [exitX, exitY] = [this.gameState.exit.x, this.gameState.exit.y];
      map[playerY][playerX] = "Player";
      map[exitY][exitX] = "Exit";

      return map.map(x => x.map(y => this.gameViz[y]).join("")).join("\n");
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
