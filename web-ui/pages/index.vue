<template>
  <div class="container">
    <AppContainer class="app-container" />
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
    AppContainer,
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
          parent.log(\`Javascript Error : \${error} on line \${line - 10}\`, 'error');
        }
      `;

      doc.open();
      doc.write(
        unescape(
          "%3Cscript%3E" +
            logger +
            "console.log(new Date(Date.now()).toLocaleTimeString());" +
            this.code +
            "console.log('\\n');" +
            "%3C/script%3E",
        ),
      );
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
