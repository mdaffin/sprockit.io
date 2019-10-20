<template>
  <div class="app">
    <GamePanel class="game-panel" />
    <EditorPanel
      class="editor-panel"
      v-model="code"
      @run="run"
      :console="console"
    />
  </div>
</template>

<script>
import EditorPanel from "~/components/EditorPanel.vue";
import GamePanel from "~/components/GamePanel.vue";

export default {
  components: {
    EditorPanel,
    GamePanel,
  },
  data() {
    return {
      code:
        "const a = 10;\nconst b = 20;\nconsole.log(a + b);\nconsole.log('A String');\n",
      console: [],
    };
  },
  mounted() {
    if (typeof Storage !== "undefined") {
      if (localStorage.code) {
        this.code = localStorage.code;
      }
      if (localStorage.editorWidth) {
        document.documentElement.style.setProperty(
          "--output-width",
          `${localStorage.editorWidth}`,
        );
      }
    }

    window.log = (output, type) => {
      this.addToLog(output, type);
    };

    const saveCode = e => {
      const modifierKey = navigator.platform.match("Mac")
        ? e.metaKey
        : e.ctrlKey;
      if (e.keyCode == 83 && modifierKey) {
        e.preventDefault();
        if (typeof Storage !== "undefined") localStorage.code = this.code;
      }
    };

    document.addEventListener("keydown", saveCode);
  },
  methods: {
    run() {
      const container = document.getElementById("editor-panel-header-handle");
      const iframe = document.createElement("IFRAME");
      container.innerHTML = "";
      iframe.style.width = "0px";
      iframe.style.height = "0px";
      iframe.style.border = "none";
      container.appendChild(iframe);

      const doc = iframe.contentDocument;

      const logger = `
        const console = {
          log:(output) => {
            parent.log(output, 'norm');
          }
        };

        window.onerror = function(error, url, line) {
          parent.log(\`Javascript Error : \${error} on line \${line}\`, 'error');
        }
      `;

      doc.open();
      doc.write(`<script>${logger}${unescape("%3C/script%3E")}`);
      doc.write(`<script>${this.code}${unescape("%3C/script%3E")}`);
      doc.close();
    },
    addToLog(output, type) {
      const consoleLine = {
        output: output,
        type: type,
      };
      this.console.push(consoleLine);
    },
  },
};
</script>

<style scoped>
:root {
  --output-width: 30%;
}

.app {
  display: grid;
  height: 100vh;
  width: 100vw;
  grid-template-columns: 1fr var(--output-width);
  grid-template-areas: "game editor";
}

.editor-panel {
  grid-area: editor;
}

.game-panel {
  grid-area: game;
}
</style>
