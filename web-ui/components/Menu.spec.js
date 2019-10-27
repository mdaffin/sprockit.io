import { mount } from "@vue/test-utils";
import Menu from "@/components/Menu";
import { createStore } from "@/tests/utils.js";

describe("Console", () => {
  test("Clicking on clear should clear the console store", () => {
    const { localVue, store } = createStore([{ output: "Some output" }]);
    const menu = mount(Menu, { localVue, store });
    menu.find(".clear-button").trigger("click");
    expect(store.state.console.lines).toEqual([]);
  });
});
