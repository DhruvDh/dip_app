<template>
  <div class="viewerDiv has-text-centered">
    <div :id="mountDiv" />
  </div>
</template>

<script>
export default {
  props: ['fileObj', 'mountDiv'],
  computed: {
    file() {
      return this.fileObj.file;
    },
    fileParseSuccessful() {
      return this.fileObj.fileParseSuccessful;
    },
  },
  watch: {
    file(newVal) {
      this.fileChanged(newVal);
    },
  },
  mounted() {
    if (this.file && this.fileParseSuccessful) {
      this.fileChanged(this.file);
    }
  },
  methods: {
    fileChanged(newVal) {
      const viewerDiv = document.getElementById(this.mountDiv);

      while (viewerDiv.lastElementChild) {
        viewerDiv.removeChild(viewerDiv.lastElementChild);
      }

      const canvas = document.createElement('canvas');
      canvas.width = newVal.width;
      canvas.height = newVal.height;
      const ctx = canvas.getContext('2d');

      ctx.putImageData(
        new ImageData(newVal.pixels, newVal.width, newVal.height),
        0,
        0,
      );

      viewerDiv.appendChild(canvas);
    },
  },
};
</script>

<style scoped>
.viewerDiv {
  padding-top: 2rem;
  justify-content: center;
}
</style>