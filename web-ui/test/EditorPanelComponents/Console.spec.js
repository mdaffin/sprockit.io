import { shallowMount } from "@vue/test-utils";
import Console from "@/components/EditorPanelComponents/Console";

describe("Console", () => {
  test("Displays console output when passed", () => {
    const wrapper = shallowMount(Console);
    wrapper.setData({ console: "This is some console output" });

    const content = wrapper.find("#console").text();

    expect(content).toEqual("This is some console output");
  });
});
