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
      <iframe ref="iframe"></iframe>
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

function genIFrameSource(code) {
  return `
    <html>
      <head>
        <script>
          const console = {
             log: (output) => {
              parent.log(output, 'norm');
            }
          };

          window.onerror = function(error, url, line) {
            parent.log(\`Javascript Error : \${error} on line \${line}\`, 'error');
          }
        <\/script>
        <script>
          ${code}
        <\/script>
      </head>
      <body></body>
    </html>
  `;
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
      this.$refs.iframe.src = URL.createObjectURL(
        new Blob([genIFrameSource(this.$store.state.script)], {
          type: "text/html",
        }),
      );
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

iframe {
  display: none;
}

header img {
  margin: 8px;
}
</style>
