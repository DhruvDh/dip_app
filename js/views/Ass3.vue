<template>
  <section class="section is-full">
    <div class="column is-narrow is-size-2 has-text-centered">Assignment 3</div>

    <ImageViewer :file-obj="$store.state.ass3" mount-div="viewerDiv" />
    <Log
      :p-errors="$store.state.ass3.fileParseErrors"
      :p-success="$store.state.ass3.fileParseSuccessful"
    />
    <FilePicker :page-name="pageName" />
  </section>
</template>

<script>
import ImageViewer from "../components/ImageViewer.vue";
import FilePicker from "../components/FilePicker.vue";
import Log from "../components/Log.vue";
import MdPage from "../md/Ass2.vue";

export default {
  components: {
    ImageViewer,
    FilePicker,
    Log,
    MdPage
  },
  computed: {
    pageName() {
      return this.$store.state.pageNameRev[this.$store.state.route.name];
    }
  }
};
</script>