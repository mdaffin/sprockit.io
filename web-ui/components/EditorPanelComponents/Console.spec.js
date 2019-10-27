import { mount } from "@vue/test-utils";
import Console from "@/components/EditorPanelComponents/Console";
import Vuex from "vuex";
import { createLocalVue } from "@vue/test-utils";
import { mutations } from "@/store/console";

describe("Console", () => {
  function createStore(lines) {
    const localVue = createLocalVue();
    localVue.use(Vuex);
    const store = new Vuex.Store({
      modules: { console: { state: { lines }, mutations } },
    });
    return { localVue, store };
  }

  test("Console output passed into the component is displayed to the user", () => {
    const consoleComponent = mount(
      Console,
      createStore([{ type: "norm", output: "This is some console output" }]),
    );
    expect(consoleComponent.html()).toContain("This is some console output");
  });

  test("Output should be blank if none has been passed in yet", () => {
    const consoleComponent = mount(Console, createStore([]));
    expect(consoleComponent.isEmpty()).toBeTruthy();
  });
});
