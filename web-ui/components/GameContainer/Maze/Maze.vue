<template>
  <div v-if="!gameState">
    Loading..... A css loader would be good
  </div>
  <div v-else>
    {{ gameState }}
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
      //this.gameState = (await this.$axios.get("/api/game/maze")).data;
      this.gameState = (await this.$axios.get(
        "https://dsdsd.free.beeceptor.com/api/game/maze",
      )).data;
    },
  },
};
</script>
