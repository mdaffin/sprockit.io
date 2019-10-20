<template>
  <div v-if="!gameState">
    Loading..... A css loader would be good
  </div>
  <div v-else>
    <div class="maze-game-ascii">
      <pre>{{ mapMaze() }}</pre>
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
  mounted: function() {
    this.createGameSession();
  },
  methods: {
    async createGameSession() {
      this.gameViz = {
        Blocked: "██",
        Exit: "▒▒",
        Open: "  ",
        Player: "⋐⋑",
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
  display: flex;
  align-items: center;
  justify-content: center;
}

.maze-game-ascii > pre {
  border: solid 4px #373737;
  font-size: 2.25vw;
}
</style>
