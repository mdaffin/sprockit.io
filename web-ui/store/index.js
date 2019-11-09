const DEFAULT_SCRIPT = `const a = 10;
const b = 20;
console.log(a + b);
console.log('A String');
`;

export const state = () => ({
  maze: Array.from({ length: 10 }, () =>
    Array.from({ length: 10 }, () => "hidden"),
  ),
  script: DEFAULT_SCRIPT,
});

export const mutations = {
  setMaze(state, maze) {
    state.maze = maze;
  },
  setScript(state, script) {
    state.script = script;
  },
};

export const actions = {
  async fetchMaze({ commit }, token) {
    const { data } = await this.$axios.get("/api/game/maze/map", {
      headers: { "X-TOKEN": token },
    });
    commit("setMaze", data);
  },
};
