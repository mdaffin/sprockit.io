export const state = () => ({
  run: false,
});

export const mutations = {
  SET_RUN(state, bool) {
    state.run = bool;
  },
};
