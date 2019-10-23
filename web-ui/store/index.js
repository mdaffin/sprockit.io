export const state = () => ({
  maze: Array.from({ length: 10 }, () =>
    Array.from({ length: 10 }, () => "hidden"),
  ),
});

export const mutations = {
  SET_MAZE(state, maze) {
    state.maze = maze;
  },
};

export const actions = {
  async FETCH_MAZE({ commit, dispatch }, token) {
    const { data } = await this.$axios.get("/api/game/maze/map", {
      headers: { "X-TOKEN": token },
    });
    commit("SET_MAZE", data.map);
    return data;
  },
};
