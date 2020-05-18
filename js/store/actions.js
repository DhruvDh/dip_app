const actions = {
  PARSE_FILE(context, { file, type }) {
    const reader = new FileReader();
    reader.readAsText(file);
    reader.onerror = (error) => console.log(error);

    if (type === 'viewer') {
      reader.onload = (event) => {
        const text = event.target.result.trim();
        try {
          const header = this._vm.lib.viewerParseHeader(text);
          header.pixels = new Uint8ClampedArray(this._vm.lib.viewerParsePixels(text));
          header.name = file.name;
          context.commit('ADD_FILE', {
            file: header,
            type: 'viewer',
          });
        } catch (errors) {
          context.commit('ADD_FILE_PARSE_ERRORS', {
            errors,
            type: 'viewer',
          });
        }
      };
    } else if (type === 'ass1') {
      reader.onload = (event) => {
        const text = event.target.result;
        try {
          const header = {
            name: file.name,
          };
          header.values = this._vm.lib.ass1ParseFile(text);
          const csvValues = this._vm.lib.ass1ConvertToCsv(text);
          const asciiValues = this._vm.lib.ass1ConvertToAsciiArt(text, '~', '8', 0.5);
          console.log(asciiValues);
          let csvFile;
          const data = new Blob([csvValues], { type: 'text/plain' });

          let asciiFile;
          const asciiData = new Blob([asciiValues], { type: 'text/plain' });
          // If we are replacing a previously generated file we need to
          // manually revoke the object URL to avoid memory leaks.
          if (csvFile !== null) {
            window.URL.revokeObjectURL(csvFile);
            window.URL.revokeObjectURL(asciiFile);
          }

          csvFile = window.URL.createObjectURL(data);
          asciiFile = window.URL.createObjectURL(asciiData);

          context.commit('ADD_CSV_FILE', csvFile);
          context.commit('ADD_ASCII_ART_FILE', asciiFile);
          context.commit('ADD_FILE', {
            file: header,
            type: 'ass1',
          });
        } catch (errors) {
          context.commit('ADD_FILE_PARSE_ERRORS', {
            errors,
            type: 'ass1',
          });
        }
      };
    }
  },
};

export default actions;
