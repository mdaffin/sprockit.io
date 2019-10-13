<template>
  <div class="container">
    <div class="output" id="output-area">
      <iframe id="output" />
    </div>
    <Editor class="editor-panel" v-model="code" @run="run" />
  </div>
</template>

<script>
import Editor from "~/components/Editor.vue";

export default {
  components: {
    Editor,
  },
  data() {
    return {
      code: "const a = 10;\nconst b = 20;\nconsole.log(a + b);\n",
    };
  },
  methods: {
    run() {
      const container = document.getElementById("output-area");
      const iframe = document.createElement("IFRAME");
      container.innerHTML = "";
      container.appendChild(iframe);

      const doc = iframe.contentDocument;
      doc.open();
      doc.write(unescape("%3Cscript%3E" + this.code + "%3C/script%3E"));
      doc.close();
    },
  },
};
</script>

<style>
:root {
  --output-width: 40%;
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
