const DEFAULT_SCRIPT = `import {Maze} from '/game/maze.js';

async function run() {
  console.log("Starting maze solver");
  const maze = new Maze();
  await maze.start();
  console.log("token: " + maze.token);
}

(async function() {run()})();
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
