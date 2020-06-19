<template>
  <section
    class="section is-full"
    @drop="dropHandler"
    @dragover="dragOverHandler"
  >
   <div class="column is-narrow is-size-2 has-text-centered">
      Image Viewer
    </div>
    <ImageViewer
      :file-obj="$store.state.viewer"
      :mount-div="mountDiv"
    />
    <Log
      :p-errors="$store.state.viewer.fileParseErrors"
      :p-success="$store.state.viewer.fileParseSuccessful"
      :f-name="fileName"
      :f-width="fileWidth"
      :f-height="fileHeight"
      :f-type="fileType"
    />
    <FilePicker :page-name="pageName" />
  </section>
</template>

<script>
import ImageViewer from '../components/ImageViewer.vue';
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
      mountDiv: 'viewerDiv',
    };
  },
  computed: {
    pageName() {
      return this.$store.state.pageNameRev[this.$store.state.route.name];
    },
    fileName() {
      if (this.$store.state.viewer.file) {
        return this.$store.state.viewer.file.name;
      }
      return '';
    },
    fileHeight() {
      if (this.$store.state.viewer.file) {
        return this.$store.state.viewer.file.height;
      }
      return '';
    },
    fileWidth() {
      if (this.$store.state.viewer.file) {
        return this.$store.state.viewer.file.width;
      }
      return '';
    },
    fileType() {
      if (this.$store.state.viewer.file) {
        return this.$store.state.viewer.file.file_type;
      }
      return '';
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
  },
};
</script>
