export const state = () => ({
  lines: [],
});

export const mutations = {
  append(state, line) {
    state.lines.unshift(line);
  },
  clear(state) {
    state.lines = [];
  },
};
