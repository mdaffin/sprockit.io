import Vuex from "vuex";
import { createLocalVue } from "@vue/test-utils";
import { mutations } from "@/store/console";

export const createStore = lines => {
  const localVue = createLocalVue();
  localVue.use(Vuex);
  const store = new Vuex.Store({
    modules: {
      console: {
        namespaced: true,
        state: { lines },
        mutations,
      },
    },
  });
  return { localVue, store };
};
