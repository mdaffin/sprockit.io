<template>
  <div @mousedown="startDrag"><span>...</span></div>
</template>

<script>
export default {
  methods: {
    startDrag(e) {
      e.preventDefault();
      window.addEventListener("mousemove", this.mouseMove);
      window.addEventListener("mouseup", this.stopDrag);
    },
    stopDrag() {
      window.removeEventListener("mousemove", this.mouseMove);
      window.removeEventListener("mouseup", this.stopDrag);
    },
    mouseMove(e) {
      e.preventDefault();
      const { offsetLeft, offsetTop } = this.$el.parentNode;
      const { offsetWidth, offsetHeight } = this.$el.parentNode;
      const { x, y } = e;

      this.$emit("drag", {
        x,
        y,
        offsetX: x - offsetLeft,
        offsetY: y - offsetTop,
        percentageX: ((x - offsetLeft) / offsetWidth) * 100,
        percentageY: ((y - offsetTop) / offsetHeight) * 100,
      });
    },
  },
};
</script>

<style scoped>
div {
  background-color: #f0f0f0;
  color: var(--background-color);
  width: 8px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: ew-resize;
}

div > span {
  transform: translateX(0.25em) rotate(90deg);
}

.panel.vertical > div > span {
  transform: translateY(-0.25em);
}

.panel.vertical > div {
  width: 100%;
  height: 8px;
  cursor: ns-resize;
}
</style>
