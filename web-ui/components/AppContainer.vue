<template>
  <div>
    <div 
      class = "game-list"
      v-if = "!selectedGame"
    >
      <div class = "game-card" 
        v-for="game in games"
        @click = setGameMode(game)
      >
        <div class = "game-card-visual gold">

        </div>
        <div>{{game}}</div>
      </div>
    </div>
    <GameManager 
      :game="selectedGame"
      v-else 
    />
  </div>
</template>

<script>
import GameManager from "~/components/GameContainer/GameManager.vue";

export default {
  components: {
    GameManager
  },
  data() {
    return {
      games: ['Maze'],
      selectedGame: null
    };
  },
  mounted: function () {

  },
  methods: {
    setGameMode(Game) {
      this.selectedGame = Game;
    }
  }
}
</script>

<style scoped>
.game-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  grid-template-rows: 190px;
  gap: 2rem 2rem;
}
.game-card {
  height:190px;
  width:100%;
  color:white;
  display:flex;
  flex-direction: column;
}

.game-card-visual {
  border-radius: 10px;
  height:170px;
}

.gold {
  background-color: #C0B283;
}
</style>


