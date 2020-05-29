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
        <ImageViewer
          :file-obj="$store.state.ass2.partA"
          mount-div="viewerDivA"
        />
        <b-button
          :disabled="convIsDisabled"
          class="is-primary"
          style="margin-top: 1.25rem;"
          @click="convToCsvClick"
        >
          Convert
        </b-button>
      </b-tab-item>

      <b-tab-item
        label="Part B"
        class="menuItem"
      >
        <ImageViewer
          :file-obj="$store.state.ass2.partB"
          mount-div="viewerDivB"
        />
      </b-tab-item>

      <b-tab-item
        label="Part C"
        class="menuItem"
      >
        <ImageViewer
          :file-obj="$store.state.ass2.partC"
          mount-div="viewerDivC"
        />
        <b-button
          :disabled="convIsDisabled"
          class="is-primary"
          @click="convToGrayImgClick"
        >
          Convert
        </b-button>
      </b-tab-item>
    </b-tabs>
    <ImageViewer
      :file-obj="$store.state.ass2"
      mount-div="viewerDiv"
    />
    <Log
      :p-errors="$store.state.ass2.fileParseErrors"
      :p-success="$store.state.ass2.fileParseSuccessful"
      :f-name="fileName"
    />
    <FilePicker :page-name="pageName" />
    <MdPage />
  </section>
</template>

<script>
import ImageViewer from '../components/ImageViewer.vue';
import FilePicker from '../components/FilePicker.vue';
import Log from '../components/Log.vue';
import MdPage from '../md/Ass2.vue';

export default {
  components: {
    ImageViewer,
    FilePicker,
    Log,
    MdPage,
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
      return !this.$store.state.ass2.fileParseSuccessful;
    },
    parseSuccessful() {
      return this.$store.state.ass2.fileParseSuccessful;
    },
    fileName() {
      if (this.$store.state.ass2.fileParseSuccessful) {
        return `${this.$store.state.ass2.partA.name}, ${this.$store.state.ass2.partB.name}, and ${this.$store.state.ass2.partC.name}`;
      }
      return '';
    },
  },
  watch: {
    grayImgValuesNotReady(newVal) {
      if (newVal) {
        const viewerDiv = document.getElementById('viewerDiv');

        while (viewerDiv.lastElementChild) {
          viewerDiv.removeChild(viewerDiv.lastElementChild);
        }
      }
    },
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
    convToCsvClick() {
      this.$store.dispatch('ASS1_CONVERT_TO_CSV', {
        imgWidth: this.$store.state.ass1.imgWidth,
      });

      const a = document.createElement('a');
      a.setAttribute('href', this.$store.state.ass1.csvFileUrl);
      a.setAttribute(
        'download',
        this.$store.state.ass1.file.name.replace('.txt', '.csv'),
      );
      a.click();
      a.remove();
    },
    convToAsciiClick() {
      this.$store.commit('TOGGLE_ASCII_MODAL_ON');
    },
    convToGrayImgClick() {
      this.$store.dispatch('ASS1_CONVERT_GRAYSCALE_IMG');
      const viewerDiv = document.getElementById('viewerDiv');

      while (viewerDiv.lastElementChild) {
        viewerDiv.removeChild(viewerDiv.lastElementChild);
      }

      const length = this.grayImgValues.length / 4;
      const height = length / this.imgWidth;

      const canvas = document.createElement('canvas');
      canvas.width = this.imgWidth;
      canvas.height = height;
      const ctx = canvas.getContext('2d');

      ctx.putImageData(
        new ImageData(this.grayImgValues, this.imgWidth, height),
        0,
        0,
      );

      viewerDiv.appendChild(canvas);
    },
  },
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
