export const state = () => ({
  run: false,
  token: "",
});

export const mutations = {
  SET_RUN(state, bool) {
    state.run = bool;
  },
  SET_TOKEN(state, token) {
    state.token = token;
  },
};
