import { mount } from "@vue/test-utils";
import HeaderButton from "./HeaderButton";

describe("ExecuteButton", () => {
  test("Should emit a event with a click property", () => {
    const button = mount(HeaderButton);
    button.trigger("click");

    expect(button.emitted().click).toBeTruthy();
  });
});
