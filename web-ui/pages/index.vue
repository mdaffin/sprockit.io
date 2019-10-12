<template>
  <div class="container">
    <div class="output" id="output-area">
      <p>script output will go here</p>
      <iframe id="output" />
    </div>
    <Editor class="editor-panel" />
  </div>
</template>

<script>
import Editor from "~/components/Editor.vue";

export default {
  components: {
    Editor,
  },
  created() {
    //For some reason bus crashes so this if statement is neccessary
    if (this.$bus) {
      this.$bus.on("update-code", this.onCmCodeChange);
      this.$bus.on("runCode", this.runCode);
    }
  },
  beforeDestroy() {
    this.$bus.off("update-code", this.onCmCodeChange);
    this.$bus.off("runCode", this.runCode);
  },
  data() {
    return {
      code: "",
    };
  },
  methods: {
    onCmCodeChange(event) {
      this.code = event;
    },
    runCode() {
      let code = this.code;

      let code_norm = code.toLowerCase();

      if (code_norm.indexOf("<script") == -1) {
        code = `<script>${code}</s` + `cript>`;
      }

      //Generate New Iframe
      document.getElementById("output").remove();

      let i = document.createElement("IFRAME");
      i.id = "output";

      let parent = document.getElementById("output-area");
      parent.appendChild(i);

      var doc = document.getElementById("output").contentDocument;
      doc.open();
      doc.write(code);
      doc.close();
    },
  },
};
</script>

<style>
:root {
  --output-width: 900px;
}

.container {
  display: grid;
  height: 100vh;
  width: 100vw;
  grid-template-columns: var(--output-width) 1fr;
  grid-template-areas: "output editor";
}

.editor-panel {
  grid-area: editor;
  display: flex;
}

.output {
  grid-area: output;
}
</style>
