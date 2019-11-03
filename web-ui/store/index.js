export const state = () => ({
  maze: Array.from({ length: 10 }, () =>
    Array.from({ length: 10 }, () => "hidden"),
  ),
});

export const mutations = {
  setMaze(state, maze) {
    state.maze = maze;
  },
};

export const actions = {
  async fetchMaze({ commit }, token) {
    const { data } = await this.$axios.get("/api/game/maze/map", {
      headers: { "X-TOKEN": token },
    });
    commit("setMaze", data);
    return data;
  },
};
