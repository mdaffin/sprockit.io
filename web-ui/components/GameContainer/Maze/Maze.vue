<template>
  <div v-if="!gameState">
    Loading..... A css loader would be good
  </div>
  <div v-else class="maze-game-ascii gold">
    <pre>{{ mapMaze() }}</pre>
  </div>
</template>

<script>
export default {
  data() {
    return {
      gameState: null,
    };
  },
  mounted: function() {
    this.createGameSession();
  },
  methods: {
    async createGameSession() {
      this.gameViz = {
        Player: "⋐⋑",
        Open: "░░",
        Blocked: "▓▓",
        Exit: "██",
      };
      //this.gameState = (await this.$axios.get("/api/game/maze")).data;
      this.gameState = {
        player: { x: 0, y: 0 },
        exit: { x: 9, y: 0 },
        map: [
          [
            "Open",
            "Open",
            "Open",
            "Open",
            "Open",
            "Open",
            "Open",
            "Open",
            "Open",
            "Open",
          ],
          [
            "Open",
            "Open",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
          ],
          [
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Blocked",
            "Open",
          ],
        ],
      };
    },
    mapMaze() {
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
  },
};
</script>

<style scoped>
.maze-game-ascii {
  border-radius: 10px;
  width: 100%;
  height: 100%;
}
.maze-game-ascii > pre {
  font-size: 2.25vw;
  margin: -5px 0;
}
</style>
