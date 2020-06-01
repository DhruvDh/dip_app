<template>
  <section class="section is-full">
    <div class="column is-narrow is-size-2 has-text-centered">
      Assignment 2
    </div>

    <b-tabs
      v-if="parseSuccessful"
      position="is-centered"
    >
      <b-tab-item
        label="Part A"
        class="menuItem"
      >
      </b-tab-item>

      <b-tab-item 
        label="Part B">
      </b-tab-item>

      <b-tab-item
        label="Part C"
        class="menuItem"
      >
      </b-tab-item>
    </b-tabs>

    <ImageViewer />
    <Log :page-name="pageName" />
    <FilePicker :page-name="pageName" />
  </section>
</template>

<script>
import ImageViewer from '../components/Ass1/Renderer.vue';
import FilePicker from '../components/FilePicker.vue';
import Log from '../components/Log.vue';

export default {
  components: {
    ImageViewer,
    FilePicker,
    Log,
  },
  data() {
    return {
      imgWidth: 256,
    };
  },
  computed: {
    pageName() {
      return this.$store.state.pageNameRev[this.$store.state.route.name];
    },
    convIsDisabled() {
      return !this.$store.state.ass1.fileParseSuccessful;
    },
    parseSuccessful() {
      return this.$store.state.ass1.fileParseSuccessful;
    },
  },
  watch: {
   
  },
  methods: {
    dropHandler(ev) {
      // Prevent default behavior (Prevent file from being opened)
      ev.preventDefault();

      if (ev.dataTransfer.items) {
        // Use DataTransferItemList interface to access the file(s)
        for (let i = 0; i < ev.dataTransfer.items.length; i += 1) {
          // If dropped items aren't files, reject them
          if (ev.dataTransfer.items[i].kind === 'file') {
            const file = ev.dataTransfer.items[i].getAsFile();
            this.$store.dispatch('PARSE_FILE', {
              file,
              type: this.pageName,
            });
          }
        }
      } else {
        // Use DataTransfer interface to access the file(s)
        for (let i = 0; i < ev.dataTransfer.files.length; i += 1) {
          console.log(
            `2. ... file[${i}].name = ${ev.dataTransfer.files[i].name}`,
          );
        }
      }
    },
    dragOverHandler(ev) {
      // Prevent default behavior (Prevent file from being opened)
      ev.preventDefault();
    },
  }
};
</script>

<style scoped>
.menuItem {
  text-align: center;
  box-sizing: unset;
  margin-left: auto;
  margin-right: auto;
}
</style>
