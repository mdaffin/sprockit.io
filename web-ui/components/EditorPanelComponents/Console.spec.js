import { mount } from "@vue/test-utils";
import Console from "@/components/EditorPanelComponents/Console";
import { createStore } from "@/tests/utils.js";

describe("Console", () => {
  test.each([
    { output: "This is some console output" },
    { type: "error", output: "This is an error" },
    { output: "This is some other output" },
    { type: "error", output: "This is another error" },
  ])(
    "Console output passed into the component is displayed to the user",
    line => {
      const consoleComponent = mount(Console, createStore({ lines: [line] }));
      expect(consoleComponent.html()).toContain(line.output);
    },
  );

  test.each([{ type: "error", output: "This is an error" }])(
    "Console errors add the error class",
    line => {
      const consoleComponent = mount(Console, createStore({ lines: [line] }));
      expect(consoleComponent.find(".console > div").classes()).toContain(
        "error",
      );
    },
  );

  test("Output should be blank if none has been passed in yet", () => {
    const consoleComponent = mount(Console, createStore([]));
    expect(consoleComponent.isEmpty()).toBeTruthy();
  });
});
