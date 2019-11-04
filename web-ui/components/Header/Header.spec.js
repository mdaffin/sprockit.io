import { mount } from "@vue/test-utils";
import Header from "./Header";
import { createStore } from "@/tests/utils.js";

describe("Console", () => {
  test("Clicking on clear should clear the console store", () => {
    const { localVue, store } = createStore({
      lines: [{ output: "Some output" }],
    });
    const menu = mount(Header, { localVue, store });
    menu.find({ ref: "clear" }).trigger("click");
    expect(store.state.console.lines).toEqual([]);
  });
});
