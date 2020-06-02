<template>
  <section class="section is-full" @drop="dropHandler" @dragover="dragOverHandler">
    <div class="column is-narrow is-size-2 has-text-centered">Assignment 3</div>

    <ImageViewer :file-obj="$store.state.ass3" :mount-div="mountDiv" />
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
        <b-input v-model="kernelText" @input="kernelTextChange"></b-input>
      </b-field>

      <Log
        v-if="!(kernelParseSuccessful)"
        :p-errors="kernelParseErrors"
        :p-success="kernelParseSuccessful"
      />

      <div class="kernelHead">
        <div class="kernel">
          <b-field>
            <b-input v-model="kernelArray[0][0]"></b-input>
          </b-field>
          <b-field>
            <b-input v-model="kernelArray[0][1]"></b-input>
          </b-field>
          <b-field>
            <b-input v-model="kernelArray[0][2]"></b-input>
          </b-field>
        </div>
        <div class="kernel">
        <b-field>
            <b-input v-model="kernelArray[1][0]"></b-input>
          </b-field>
          <b-field>
            <b-input v-model="kernelArray[1][1]"></b-input>
          </b-field>
          <b-field>
            <b-input v-model="kernelArray[1][2]"></b-input>
          </b-field>
        </div>
        <div class="kernel">
          <b-field>
            <b-input v-model="kernelArray[2][0]"></b-input>
          </b-field>
          <b-field>
            <b-input v-model="kernelArray[2][1]"></b-input>
          </b-field>
          <b-field>
            <b-input v-model="kernelArray[2][2]"></b-input>
          </b-field>
        </div>
      </div>
      
    </div>
    <FilePicker :page-name="pageName" />
    <MdPage />
  </section>
</template>

<script>
import ImageViewer from "../components/ImageViewer.vue";
import FilePicker from "../components/FilePicker.vue";
import Log from "../components/Log.vue";
import KernelFormat from "../md/KernelFormat.vue";
import MdPage from "../md/Ass3.vue";

export default {
  components: {
    ImageViewer,
    FilePicker,
    Log,
    MdPage,
    KernelFormat
  },
  data() {
    return {
      mountDiv: "ass3Div",
      kernelParseSuccessful: true,
      kernelParseErrors: "",
      kernelText: "[[1.2, 1.2, 1.2], [1.2, 1.2, 1.2], [1.2, 1.2, 1.2]]",
      kernelArray: [[1.2, 1.2, 1.2], [1.2, 1.2, 1.2], [1.2, 1.2, 1.2]],
    };
  },
  watch: {
      kernelText(newVal) {
          console.log(newVal);
      }
  },
  computed: {
    pageName() {
      return this.$store.state.pageNameRev[this.$store.state.route.name];
    },
    fileName() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.name;
      }
      return "";
    },
    fileHeight() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.height;
      }
      return "";
    },
    fileWidth() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.width;
      }
      return "";
    },
    fileType() {
      if (this.$store.state.ass3.file) {
        return this.$store.state.ass3.file.file_type;
      }
      return "";
    }
  },
  methods: {
    dropHandler(ev) {
      // Prevent default behavior (Prevent file from being opened)
      ev.preventDefault();

      if (ev.dataTransfer.items) {
        // Use DataTransferItemList interface to access the file(s)
        for (let i = 0; i < ev.dataTransfer.items.length; i += 1) {
          // If dropped items aren't files, reject them
          if (ev.dataTransfer.items[i].kind === "file") {
            const file = ev.dataTransfer.items[i].getAsFile();
            this.$store.dispatch("PARSE_FILE", {
              file,
              type: this.pageName
            });
          }
        }
      } else {
        // Use DataTransfer interface to access the file(s)
        for (let i = 0; i < ev.dataTransfer.files.length; i += 1) {
          console.log(
            `2. ... file[${i}].name = ${ev.dataTransfer.files[i].name}`
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
        this.lib.ass3ParseKernel(newVal);
      } catch (err) {
        this.kernelParseSuccessful = false;
        this.kernelParseErrors = err;
      }
    }
  }
};
</script>

<style scoped>
.kernel {
  width: 5%;
}

.kernelHead {
  display: flex;
  margin: 2rem;
}
</style>