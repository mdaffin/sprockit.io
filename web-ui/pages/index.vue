<template>
  <div class="container">
    <EditorPanel class="editor-container" v-model="code" @run="run" />
  </div>
</template>

<script>
import EditorPanel from "~/components/EditorPanel.vue";

export default {
  components: {
    EditorPanel,
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
  --output-width: 30%;
}

.container {
  display: grid;
  height: 100vh;
  width: 100vw;
  grid-template-columns: 1fr var(--output-width);
  grid-template-areas: "output editor";
}

.editor-container {
  grid-area: editor;
  display: flex;
}

.output {
  grid-area: output;
  background: #151515;
  width: 100%;
}
</style>
