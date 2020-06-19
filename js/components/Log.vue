<template>
  <div class="container">
    <article
      id="logContainer"
      :class="logMessageClass"
    >
      <div
        id="hint"
        class="message-header"
      >
        <p>Log</p>
      </div>
      <div
        id="errorLog"
        class="message-body"
      >
        <div v-if="pSuccess">
          <p
            v-for="msg in logMessage"
            :key="msg"
          >
            {{ msg }}
          </p>
        </div>
        <pre v-else>{{ logMessage }}</pre>
      </div>
    </article>
  </div>
</template>

<script>
export default {
  props: ['pErrors', 'pSuccess', 'fName', 'fHeight', 'fWidth', 'fType'],
  computed: {
    fileParseErrors() {
      let errors;
      if (this.pErrors) {
        if (this.pErrors.split) {
          errors = this.pErrors.split(
            '#!@',
          );
        } else {
          errors = [this.pErrors];
        }

        if (errors.length >= 15) {
          errors = errors.slice(0, 15);
          errors.push('😭 And I stopped counting after the first 15 errors');
        }

        return errors.join('\n');
      }
      return undefined;
    },
    logMessageClass() {
      if (this.pSuccess) return 'message is-primary';
      if (this.fileParseErrors) return 'message is-danger';
      return 'message is-warning';
    },
    logMessage() {
      if (this.pSuccess) {
        if (this.$route.name === 'Viewer') {
          return [
            'File parsed successfully.',
            `The parsed file type is ${
              this.fType
            }`,
            `The parsed height is ${
              this.fHeight
            }`,
            `The parsed width is ${this.fWidth}`,
          ];
        }
        if (this.$route.name === 'Assignment 1') {
          return [
            `Successfully parsed ${this.fName}`,
          ];
        }
        if (this.$route.name === 'Assignment 2') {
          return [
            `Successfully parsed ${this.fName}`,
          ];
        }
      }
      if (this.pErrors) {
        return this.fileParseErrors;
      }
      return undefined;
    },
  },
};
</script>