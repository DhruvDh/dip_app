<template>
  <div class="container">
    <section>
      <b-field>
        <b-upload v-model="dropFile" drag-drop @input="parse">
          <section class="section">
            <div class="content has-text-centered">
              <p>
                <b-icon icon="upload" size="is-large" />
              </p>
              <p v-if="dropFile" class="is-size-4">Successfully read {{ dropFile.name }}</p>
              <p v-else class="is-size-4">Drop your files here or click to upload</p>
            </div>
          </section>
        </b-upload>
      </b-field>
    </section>
    <b-button type="is-dark" class="sample" outlined expanded>Click here to load Sample file</b-button>
  </div>
</template>

<script>
export default {
  props: ["pageName"],
  data() {
    return {
      dropFile: undefined
    };
  },
  methods: {
    parse() {
      this.$store.dispatch("PARSE_FILE", {
        file: this.dropFile,
        type: this.pageName
      });
    }
  }
};
</script>

<style scoped>
.upload {
  width: 100%;
  display: block;
}

.sample {
  border-bottom: 1px dashed #b5b5b5 !important;
  border-left: 1px dashed #b5b5b5 !important;
  border-right: 1px dashed #b5b5b5 !important;
  border-top: none;
}
</style>