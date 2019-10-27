import { mount } from "@vue/test-utils";
import ExecuteButton from "@/components/EditorPanelComponents/ExecuteButton";

describe("ExecuteButton", () => {
  test("Should emit a event with a click property", () => {
    const button = mount(ExecuteButton);
    button.trigger("click");

    expect(button.emitted().click).toBeTruthy();
  });
});
