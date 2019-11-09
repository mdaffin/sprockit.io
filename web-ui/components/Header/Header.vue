<template>
  <header>
    <a href="/">
      <img src="/logo-192x32.png" alt="sprockit.io logo" />
    </a>
    <div class="button-group">
      <HeaderButton class="primary" @click="run()">Run</HeaderButton>
      <HeaderButton ref="clear" @click="$store.commit('console/clear')">
        Clear
      </HeaderButton>
      <div ref="iframe"></div>
    </div>
  </header>
</template>

<script>
import HeaderButton from "./HeaderButton";

function PressCtrlPlusKey({ key, keyCode, fun }) {
  document.addEventListener("keydown", e => {
    const modifierKey = navigator.platform.match("Mac") ? e.metaKey : e.ctrlKey;
    if ((e.key ? e.key === key : e.keyCode === keyCode) && modifierKey) {
      fun(e);
    }
  });
}

export default {
  components: { HeaderButton },
  mounted() {
    // run run() on <C-Enter>
    PressCtrlPlusKey({
      key: "Enter",
      keyCode: 13,
      fun: e => {
        e.preventDefault();
        this.run();
      },
    });
    // clear console on <C-l>
    PressCtrlPlusKey({
      key: "l",
      keyCode: 76,
      fun: e => {
        e.preventDefault();
        this.$store.commit("console/clear");
      },
    });
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
      doc.write(
        `<script>${this.$store.state.script}${unescape("%3C/script%3E")}`,
      );
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
header {
  display: flex;
  justify-content: space-between;
  height: 48px;
}

header img {
  margin: 8px;
}
</style>
