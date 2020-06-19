<template>
  <section
    class="section is-full"
    @drop="dropHandler"
    @dragover="dragOverHandler"
  >
    <div class="column is-narrow is-size-2 has-text-centered">
      Assignment 3B
    </div>

    <ImageViewer
      :file-obj="$store.state.ass3"
      :mount-div="mountDiv"
    />
    <Log
      :p-errors="$store.state.ass3.fileParseErrors"
      :p-success="$store.state.ass3.fileParseSuccessful"
      :f-name="fileName"
      :f-width="fileWidth"
      :f-height="fileHeight"
      :f-type="fileType"
    />

    <div class="container">
      <KernelFormat />

      <b-field label="Enter Kernel">
        <b-input
          v-model="kernelText"
          custom-class="kernelInput"
          @input="kernelTextChange"
        />
      </b-field>

      <div
        id="matrixDisplay"
        class="section has-text-centered"
      />

      <Log
        v-if="!(kernelParseSuccessful)"
        :p-errors="kernelParseErrors"
        :p-success="kernelParseSuccessful"
      />
    </div>
    <FilePicker
      :page-name="pageName"
      :kernel="kernel"
    />
    <MdPage />
  </section>
</template>

<script>
import { mapState } from 'vuex';
import ImageViewer from '../components/ImageViewer.vue';
import FilePicker from '../components/FilePicker.vue';
import Log from '../components/Log.vue';
import KernelFormat from '../md/KernelFormat.vue';
import MdPage from '../md/Ass3.vue';

export default {
  components: {
    ImageViewer,
    FilePicker,
    Log,
    MdPage,
    KernelFormat,
  },
  data() {
    return {
      mountDiv: 'ass3Div',
      kernelParseSuccessful: true,
      kernelParseErrors: '',
      kernelText:
        '[ [-1.0, -1.0, -1.0], [-1.0, 8.0, -1.0], [-1.0, -1.0, -1.0] ]',
    };
  },
  computed: {
    pageName() {
      return this.$store.state.pageNameRev[this.$store.state.route.name];
    },
    fileName() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.name;
      }
      return '';
    },
    fileHeight() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.height;
      }
      return '';
    },
    fileWidth() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.width;
      }
      return '';
    },
    fileType() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.file_type;
      }
      return '';
    },
    matrixMathML() {
      const rows = this.kernel.kernel.map((row) => row.join(' & '));
      const matrix = rows.join('\\\\\n');

      let scaleFactor = this.kernel.kernel.flat().reduce((a, b) => a + b, 0);
      scaleFactor = scaleFactor !== 0 ? scaleFactor.toFixed(2) : 1;
      return String.raw`\frac{1}{${scaleFactor}} *
      \begin{bmatrix}
      ${matrix}
      \end{bmatrix}`;
    },
    ...mapState({
      kernel: (state) => state.ass3.kernel,
    }),
  },
  watch: {
    kernelText(newVal) {
      this.updateMatrixDiv();
    },
  },
  mounted() {
    this.kernelTextChange(this.kernelText);
    setTimeout(() => this.updateMatrixDiv(), 1000);
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
    kernelTextChange(newVal) {
      try {
        const res = this.lib.ass3ParseKernel(newVal);
        this.kernelParseSuccessful = true;
        this.kernelParseErrors = '';
        for (let i = 0; i < res.height; i += 1) {
          const row = [];
          for (let j = 0; j < res.width; j += 1) {
            row.push(res.value[i * res.width + j]);
          }
          res.kernel.push(row);
        }
        this.$store.commit('ADD_ASS3_KERNEL', res);
        if (this.$store.state.ass3.fileParseSuccessful) {
          this.$store.dispatch('ASS3_UPDATE_IMAGE', {
            name: this.$store.state.ass3.file.name,
            type: 'ass3',
          });
        }
        return res;
      } catch (e) {
        const res = {
          kernelParseErrors: e,
          kernelParseSuccessful: false,
          height: 0,
          width: 0,
          value: [],
          kernel: [[]],
        };
        this.$store.commit('ADD_ASS3_KERNEL', res);
        this.kernelParseSuccessful = false;
        this.kernelParseErrors = e;
        const div = document.getElementById('matrixDisplay');
        div.innerHTML = '';
        MathJax.startup.document.clear();
        MathJax.startup.document.updateDocument();
        return res;
      }
    },
    updateMatrixDiv() {
      const div = document.getElementById('matrixDisplay');
      div.innerHTML = '';
      div.appendChild(MathJax.tex2chtml(this.matrixMathML));
      MathJax.startup.document.clear();
      MathJax.startup.document.updateDocument();
    },
  },
};
</script>

<style>
@font-face {
  font-family: "LM Roman 10";
  src: url("/font/lmroman10-regular.otf");
}

.kernel {
  width: 15%;
}

.kernelHead {
  display: flex;
  margin: 2rem;
}

.kernelInput {
  font-family: "LM Roman 10" !important;
  font-size: 1.75rem !important;
  text-align: center !important;
}

.MathJax {
  font-size: 2em !important;
  color: black !important;
}
</style>