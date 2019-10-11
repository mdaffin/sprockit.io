<template>
  <div class="container">
    <div class = "codemirror">
      <client-only placeholder="Codemirror Loading...">
        <codemirror v-model="code" 
                    :options="cmOption"
        />
      </client-only>
    </div>
  </div>
</template>

<script>
  export default {
    data() {
      return {
        code: 'const A = 10',
        cmOption: {
          tabSize: 4,
          styleActiveLine: true,
          lineNumbers: true,
          line: true,
          foldGutter: true,
          styleSelectedText: true,
          mode: 'text/javascript',
          keyMap: "sublime",
          matchBrackets: true,
          showCursorWhenSelecting: true,
          theme: "monokai",
          extraKeys: { "Ctrl": "autocomplete" },
          hintOptions:{
            completeSingle: false
          }
        }
      }
    },
    methods: {
      
    },
    mounted() {
      console.log('this is current codemirror object')
      // you can use this.codemirror to do something...
    }
  }
</script>

<style>
.container {
  margin: 0;
  min-height: 100vh;
  /*display: flex;*/
  justify-content: center;
  align-items: center;
  text-align: center;
}
.vue-codemirror {
  height:100vh;
  float:right;
  width:1000px;
}
.CodeMirror {
  height:100% !important;
  width:100%;
  text-align:left;
}
.title {
  font-family: "Quicksand", "Source Sans Pro", -apple-system, BlinkMacSystemFont,
    "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  display: block;
  font-weight: 300;
  font-size: 100px;
  color: #35495e;
  letter-spacing: 1px;
}

.subtitle {
  font-weight: 300;
  font-size: 42px;
  color: #526488;
  word-spacing: 5px;
  padding-bottom: 15px;
}

.links {
  padding-top: 15px;
}
</style>
