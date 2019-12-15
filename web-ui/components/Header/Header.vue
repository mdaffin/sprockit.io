<template>
  <header>
    <a href="/">
      <img src="/logo-192x32.png" alt="sprockit.io logo" />
    </a>
    <div class="button-group">
      <HeaderButton class="primary" @click="run()">
        Run
      </HeaderButton>
      <HeaderButton class="cyan" ref="save" @click="saveScript()">
        Save
      </HeaderButton>
      <HeaderButton class="purple" @click="resetScript()">
        Reset
      </HeaderButton>
      <HeaderButton
        class="rose"
        ref="clear"
        @click="$store.commit('console/clear')"
      >
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

export default {
  components: { HeaderButton },
  mounted() {
    // save script in localStorage
    PressCtrlPlusKey({
      key: "s",
      keyCode: 83,
      fun: e => {
        e.preventDefault();
        this.saveScript();
      },
    });
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
      const iframe = this.$refs.iframe;
      iframe.src = "/game/maze.html";
      iframe.onload = () => {
        const body = iframe.contentWindow.document.querySelector("body");
        const script = document.createElement("script");
        script.type = "module";
        script.text = this.$store.state.script;
        body.appendChild(script);
      };
    },
    writeToConsole(msg) {
      this.$store.commit("console/append", {
        output: msg,
      });
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
    saveScript() {
      if (typeof Storage !== "undefined") {
        localStorage.mazeScript = this.$store.state.script;
        this.writeToConsole("Saving script");
      }
    },
    resetScript() {
      this.$store.commit("resetScript");
      this.writeToConsole("Resetting script");
    },
  },
};
</script>

<style scoped>
header {
  display: flex;
  justify-content: space-between;
  height: 48px;
  background-color: var(--dark-grey);
}

.button-group {
  background-color: var(--background-color);
  display: flex;
}

iframe {
  display: none;
}

header img {
  margin: 8px;
  padding-left: 0.5em;
}
</style>
