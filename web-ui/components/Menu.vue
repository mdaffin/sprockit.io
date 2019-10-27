<template>
  <header>
    <div class="button-group">
      <h1>Sprockit.io</h1>
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

export default {
  components: { HeaderButton },
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
</style>
