import { mount } from "@vue/test-utils";
import Editor from "@/components/EditorPanelComponents/Editor";

describe("Editor", () => {
  test.each(["Mac-Intel", "Linux x86_64"])(
    "If user hits ctrl + s the code should be saved to local storage",
    platform => {
      mount(Editor, {
        propsData: {
          value: "this is a test",
        },
      });
      Object.defineProperty(window.navigator, "platform", {
        value: "",
        writable: true,
      });
      navigator.platform = platform;

      var event = new KeyboardEvent("keydown", {
        keyCode: 83,
        ctrlKey: platform !== "Mac-Intel",
        metaKey: platform === "Mac-Intel",
      });
      document.dispatchEvent(event);

      expect(localStorage.getItem("code")).toEqual("this is a test");
    },
  );
});
