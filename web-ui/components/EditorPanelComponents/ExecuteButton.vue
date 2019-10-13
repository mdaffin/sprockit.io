<template>
  <div @click="$emit('click')" class="executeButton">
    Run
  </div>
</template>

<style scoped>
.executeButton {
  height: 35px;
  width: 35px;
  color: white;
  background: green;
  cursor: pointer;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
