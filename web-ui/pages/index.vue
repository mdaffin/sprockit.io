<template>
  <div class="container">
    <AppContainer class = "app-container" />
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
import AppContainer from "~/components/AppContainer.vue";

export default {
  components: {
    EditorPanel,
    AppContainer
  },
  mounted() {
    window.log = output => {
      this.addToLog(output);
    };
  },
  data() {
    return {
      code:
        "const a = 10;\nconst b = 20;\nconsole.log(a + b);\nconsole.log('A String');\n",
      console: "",
    };
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
        var console = {
          log:(output) => {
            parent.log(output);
          }
        };
      `;

      doc.open();
      doc.write(
        unescape("%3Cscript%3E" + logger + this.code + "%3C/script%3E"),
      );
      doc.close();
    },
    addToLog(output) {
      this.console += `${output}<br>`;
    },
  },
};
</script>

<style>
:root {
  --output-width: 30%;
}

.container {
  display: grid;
  height: 100vh;
  width: 100vw;
  grid-template-columns: 1fr var(--output-width);
  grid-template-areas: "app editor";
}

.editor-panel {
  grid-area: editor;
  display: flex;
}

.app-container {
  grid-area: app;
  padding: 3rem calc(3vw + 35px) 3rem 3vw;
}
</style>
