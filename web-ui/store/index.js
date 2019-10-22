export const state = () => ({
  run: false,
  token: "",
  maze: {},
});

export const mutations = {
  SET_RUN(state, bool) {
    state.run = bool;
  },
  SET_TOKEN(state, token) {
    state.token = token;
  },
  SET_MAZE(state, maze) {
    state.maze = maze;
  },
};

export const actions = {
  async FETCH_TOKEN({ commit }) {
    const { data } = await this.$axios.post("/api/game/maze/start", "404");
    commit("SET_TOKEN", data.token);
  },
  async FETCH_MAZE({ commit, state }) {
    const { data } = await this.$axios.get("/api/game/maze/map", {
      headers: { "X-TOKEN": state.token },
    });
    commit("SET_MAZE", data);
    return data;
  },
};
