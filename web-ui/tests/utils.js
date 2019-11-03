import Vuex from "vuex";
import { createLocalVue } from "@vue/test-utils";
import { mutations as consoleMutations } from "@/store/console";
import { mutations } from "@/store";

export const createStore = ({ script, lines }) => {
  const localVue = createLocalVue();
  localVue.use(Vuex);
  const store = new Vuex.Store({
    state: { script },
    mutations,
    modules: {
      console: {
        namespaced: true,
        state: { lines },
        mutations: consoleMutations,
      },
    },
  });
  return { localVue, store };
};
