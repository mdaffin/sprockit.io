<template>
  <div class="container">
    <div class="output">
      <p>script output will go here</p>
    </div>
    <Editor :code="code" class="editor" />
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
};
</script>

<style>
:root {
  --output-width: 500px;
}

.container {
  display: grid;
  height: 100vh;
  width: 100vw;
  grid-template-columns: var(--output-width) 1fr;
  grid-template-areas: "output editor";
}

.editor {
  grid-area: editor;
}

.output {
  grid-area: output;
}
</style>
