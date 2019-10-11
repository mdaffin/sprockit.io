<template>
  <div class="container">
    <div class = "editor-panel">
      <div class = "editor-panel-handle">
        <div class = "editor-panel-handle-tab" v-bind:class="{ isSelected: tab === 'Script' }">
          <span>Script</span>
        </div>
        <div class = "editor-panel-handle-tab" v-bind:class="{ isSelected: tab === 'Console' }">
          <span>Console</span>
        </div>
      </div>
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
        tab: 'Script',
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
.editor-panel {
  display:flex;
  width:50%;
  float:right;
}

.editor-panel-handle-tab {
  width:35px;
  height:100px;
  background:#2979D3;
  color:white;
  display:flex;
}
.editor-panel-handle-tab span{
  height:35px;
  min-width:100px;
  padding:5px;
  transform-origin: 0 0;
  transform: rotate(-90deg) translate(-100px, 0);
}

.isSelected {
  background: #542FBF;
}

.container {
  margin: 0;
  min-height: 100vh;
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
  height:100%;
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
