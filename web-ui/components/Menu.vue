<template>
  <header>
    <div class="button-group">
      <img src="/logo.png" alt="sprockit.io logo" class="header-logo" />
    </div>
    <div class="button-group">
      <HeaderButton @click="$emit('run')">
        <span>Run</span>
      </HeaderButton>
      <HeaderButton
        class="clear-button"
        @click="$store.commit('console/clear')"
      >
        <span>Clear</span>
      </HeaderButton>
    </div>
  </header>
</template>

<script>
import HeaderButton from "~/components/EditorPanelComponents/HeaderButton";

function PressCtrlPlusKey({ key, keyCode, fun }) {
  return document.addEventListener("keydown", e => {
    console.log(e);
    const modifierKey = navigator.platform.match("Mac") ? e.metaKey : e.ctrlKey;
    if ((e.key === key || e.keyCode === keyCode) && modifierKey) {
      fun(e);
    }
  });
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
        this.$emit("run");
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
  },
};
</script>

<style scoped>
header {
  display: flex;
  justify-content: space-between;
  padding-left: 16px;
  padding-right: 16px;
}

.button-group {
  display: flex;
}

.header-logo {
  padding-top: 5px;
  height: 100%;
}
</style>
