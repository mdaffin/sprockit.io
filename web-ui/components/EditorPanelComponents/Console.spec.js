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

  test.each([
    { output: "This is some console output" },
    { type: "error", output: "This is an error" },
    { output: "This is some other output" },
    { type: "error", output: "This is another error" },
  ])(
    "Console output passed into the component is displayed to the user",
    line => {
      const consoleComponent = mount(Console, createStore([line]));
      expect(consoleComponent.html()).toContain(line.output);
    },
  );

  test.each([{ type: "error", output: "This is an error" }])(
    "Console errors add the error class",
    line => {
      const consoleComponent = mount(Console, createStore([line]));
      expect(consoleComponent.find(".console-line").classes()).toContain(
        "error",
      );
    },
  );

  test("Output should be blank if none has been passed in yet", () => {
    const consoleComponent = mount(Console, createStore([]));
    expect(consoleComponent.isEmpty()).toBeTruthy();
  });
});
