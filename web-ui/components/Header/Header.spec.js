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

  test("Clicking on save should save the script to local storage", () => {
    const { localVue, store } = createStore({
      script: "this is a test",
      lines: [{ output: "Some output" }],
    });
    const menu = mount(Header, { localVue, store });
    menu.find({ ref: "save" }).trigger("click");
    expect(localStorage.getItem("mazeScript")).toEqual("this is a test");
  });

  test.each(["Mac-Intel", "Linux x86_64"])(
    "If user hits ctrl + s the code should be saved to local storage",
    platform => {
      const { localVue, store } = createStore({
        script: "this is a test",
        lines: [{ output: "Some output" }],
      });
      mount(Header, { localVue, store });

      Object.defineProperty(window.navigator, "platform", {
        value: "",
        writable: true,
      });
      navigator.platform = platform;

      const keyPress = new KeyboardEvent("keydown", {
        keyCode: 83,
        ctrlKey: platform !== "Mac-Intel",
        metaKey: platform === "Mac-Intel",
      });
      document.dispatchEvent(keyPress);

      expect(localStorage.getItem("mazeScript")).toEqual("this is a test");
    },
  );
});
