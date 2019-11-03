import { mount } from "@vue/test-utils";
import Editor from "@/components/EditorPanelComponents/Editor";
import { createStore } from "@/tests/utils.js";

describe("Editor", () => {
  test.each(["Mac-Intel", "Linux x86_64"])(
    "If user hits ctrl + s the code should be saved to local storage",
    platform => {
      const { localVue, store } = createStore({ script: "this is a test" });
      mount(Editor, { localVue, store });
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
