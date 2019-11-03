<template>
  <div class="app">
    <Menu class="header" @run="run" />
    <GamePanel class="game-panel" />
    <EditorPanel class="editor-panel" v-model="code" :console="console" />
  </div>
</template>

<script>
import Menu from "~/components/Menu.vue";
import EditorPanel from "~/components/EditorPanel.vue";
import GamePanel from "~/components/GamePanel.vue";

export default {
  components: {
    Menu,
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
    if (typeof Storage !== "undefined" && localStorage.code) {
      this.code = localStorage.code;
    }
    window.log = (output, type) => {
      this.addToLog(output, type);
    };
  },
  methods: {
    async run() {
      const container = document.getElementById("editor-panel-header-handle");
      const iframe = document.createElement("IFRAME");
      const token = await this.fetch_token();
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
      this.$store.dispatch("fetchMaze", token);
    },
    addToLog(output, type) {
      const consoleLine = {
        output: output,
        type: type,
      };
      this.$store.commit("console/append", consoleLine);
    },
    async fetch_token() {
      const { data } = await this.$axios.post(
        "/api/game/maze/start",
        `${Date.now()}`,
      );
      return data.token;
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
  grid-template-columns: 1fr var(--output-width);
  grid-template-rows: 42px 1fr;
  grid-template-areas:
    "header header"
    "game editor";
}

.editor-panel {
  grid-area: editor;
}

.game-panel {
  grid-area: game;
}

.header {
  grid-area: header;
}
</style>
