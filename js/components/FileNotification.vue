<template>
  <div
    v-if="parseSuccessful"
    class="notification is-primary notify"
  >
    {{ message }}
  </div>
</template>

<script>
export default {
  props: ['page'],
  computed: {
    parseSuccessful() {
      if (this.page === 'ass1') {
        return this.$store.state.ass1.fileObj.fileParseSuccessful;
      }

      return this.$store.state[this.page].fileParseSuccessful;
    },
    message() {
      if (this.page === 'ass1') {
        return `Successfully parsed ${this.$store.state.ass1.fileObj.file.name}`;
      } if (this.page === 'ass2') {
        return `Successfully created ${this.$store.state.ass2.partA.name}, ${this.$store.state.ass2.partB.name}, and ${this.$store.state.ass2.partC.name}`;
      }

      return `Successfully parsed ${this.$store.state[this.page].file.name}`;
    },
  },
};
</script>

<style scoped>
.notify {
  padding: 0.8rem 0rem 0.8rem 0rem !important;
  margin-bottom: 0px !important;
  text-align: center;
}
</style>