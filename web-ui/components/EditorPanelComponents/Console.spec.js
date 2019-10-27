import { mount } from "@vue/test-utils";
import Console from "@/components/EditorPanelComponents/Console";

describe("Console", () => {
  test("Console output passed into the component is displayed to the user", () => {
    const wrapper = mount(Console, {
      propsData: {
        console: [{ output: "This is some console output", type: "norm" }],
      },
    });

    const content = wrapper.html();

    expect(content).toContain("This is some console output");
  });

  test("Output should be blank if none has been passed in yet", () => {
    const console = mount(Console);

    expect(console.isEmpty()).toBeTruthy();
  });
});
