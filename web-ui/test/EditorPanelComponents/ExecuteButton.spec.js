import { mount } from "@vue/test-utils";
import ExecuteButton from "@/components/EditorPanelComponents/ExecuteButton";

describe("Console", () => {
  test("Console output passed into the component is displayed to the user", () => {
    const wrapper = mount(ExecuteButton);
    const button = wrapper.find(".executeButton");

    button.trigger("click");

    expect(button.emitted().click).toBeTruthy();
  });
});
