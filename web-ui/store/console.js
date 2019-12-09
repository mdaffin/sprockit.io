export const state = () => ({
  lines: [],
});

export const mutations = {
  append(state, line) {
    state.lines.push(line);
  },
  clear(state) {
    state.lines = [];
  },
};
