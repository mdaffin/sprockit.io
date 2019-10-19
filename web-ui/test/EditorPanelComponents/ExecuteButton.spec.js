import { mount } from "@vue/test-utils";
import ExecuteButton from "@/components/EditorPanelComponents/ExecuteButton";

describe("ExecuteButton", () => {
  test("Should emit a event with a click property", () => {
    const wrapper = mount(ExecuteButton);
    const button = wrapper.find(".executeButton");

    button.trigger("click");

    expect(button.emitted().click).toBeTruthy();
  });
});
