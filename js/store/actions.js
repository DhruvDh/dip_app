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
            type,
          });
        } catch (errors) {
          context.commit('ADD_FILE_PARSE_ERRORS', {
            errors,
            type,
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
          header.text = text;
          this._vm.lib.ass1ParseFile(text);
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
    } else if (type === 'ass2') {
      reader.onload = (event) => {
        const text = event.target.result;
        try {
          const partAText = file.name.includes('_hard') ? this._vm.lib.ass2DoPartA(text, 1024) : this._vm.lib.ass2DoPartA(text, 512);
          const partBText = file.name.includes('_hard') ? this._vm.lib.ass2DoPartB(text, 1024) : this._vm.lib.ass2DoPartB(text, 512);
          const partCText = file.name.includes('_hard') ? this._vm.lib.ass2DoPartC(text, 1024) : this._vm.lib.ass2DoPartC(text, 512);

          const partAHeader = this._vm.lib.viewerParseHeader(partAText);
          const partBHeader = this._vm.lib.viewerParseHeader(partBText);
          const partCHeader = this._vm.lib.viewerParseHeader(partCText);

          partAHeader.pixels = new Uint8ClampedArray(this._vm.lib.viewerParsePixels(partAText));
          partBHeader.pixels = new Uint8ClampedArray(this._vm.lib.viewerParsePixels(partBText));
          partCHeader.pixels = new Uint8ClampedArray(this._vm.lib.viewerParsePixels(partCText));

          partAHeader.name = file.name.replace('.txt', '_out_a.txt');
          partBHeader.name = file.name.replace('.txt', '_out_b.txt');
          partCHeader.name = file.name.replace('.txt', '_out_c.txt');

          const partAData = new Blob([partAText], { type: 'text/plain' });
          const partBData = new Blob([partBText], { type: 'text/plain' });
          const partCData = new Blob([partCText], { type: 'text/plain' });

          partAHeader.fileUrl = window.URL.createObjectURL(partAData);
          partBHeader.fileUrl = window.URL.createObjectURL(partBData);
          partCHeader.fileUrl = window.URL.createObjectURL(partCData);

          context.commit('ADD_FILE', {
            file: {
              A: partAHeader,
              B: partBHeader,
              C: partCHeader,
            },
            type: 'ass2',
          });
        } catch (errors) {
          context.commit('ADD_FILE_PARSE_ERRORS', {
            errors,
            type: 'ass2',
          });
        }
      };
    } else if (type === 'ass3') {
      reader.onload = (event) => {
        const text = event.target.result.trim();
        context.commit('ADD_ASS3_FILE_TEXT', text);
        context.dispatch('ASS3_UPDATE_IMAGE', { name: file.name, type });
      };
    }
  },
  ASS1_CONVERT_TO_ASCII(context, {
    lightChar, darkChar, threshold, imgWidth,
  }) {
    const { text } = context.state.ass1.fileObj.file;
    try {
      const asciiValues = this._vm.lib.ass1ConvertToAsciiArt(text, lightChar, darkChar, threshold, imgWidth);
      const { asciiFileUrl } = context.state.ass1;
      const asciiData = new Blob([asciiValues], { type: 'text/plain' });

      if (asciiFileUrl !== null) {
        window.URL.revokeObjectURL(asciiFileUrl);
      }

      context.commit('ADD_ASCII_ART_FILE', window.URL.createObjectURL(asciiData));
    } catch (errors) {
      context.commit('ADD_FILE_PARSE_ERRORS', {
        errors,
        type: 'ass1',
      });
    }
  },
  ASS1_CONVERT_TO_CSV(context, { imgWidth }) {
    const { text } = context.state.ass1.fileObj.file;
    try {
      const csvValues = this._vm.lib.ass1ConvertToCsv(text, imgWidth);
      const { csvFileUrl } = context.state.ass1;
      const csvData = new Blob([csvValues], { type: 'text/plain' });

      if (csvFileUrl !== null) {
        window.URL.revokeObjectURL(csvFileUrl);
      }

      context.commit('ADD_CSV_FILE', window.URL.createObjectURL(csvData));
    } catch (errors) {
      context.commit('ADD_FILE_PARSE_ERRORS', {
        errors,
        type: 'ass1',
      });
    }
  },
  ASS1_CONVERT_GRAYSCALE_IMG(context) {
    const { text } = context.state.ass1.fileObj.file;
    try {
      const img = this._vm.lib.ass1ConvertToGrayscaleImg(text);
      context.commit('ADD_GRAY_IMG_VALUES', new Uint8ClampedArray(img));
    } catch (errors) {
      context.commit('ADD_FILE_PARSE_ERRORS', {
        errors,
        type: 'ass1',
      });
    }
  },
  ASS3_UPDATE_IMAGE(context, { name, type }) {
    try {
      const text = context.state.ass3.fileText;
      const { kernel } = context.state.ass3;
      console.log(kernel);
      const header = this._vm.lib.viewerParseHeader(text);
      header.pixels = new Uint8ClampedArray(this._vm.lib.ass3Compute(text, kernel.height, kernel.width, kernel.value));
      header.height = header.height - kernel.height + 1;
      header.width = header.width - kernel.width + 1;
      header.name = name;

      context.commit('ADD_FILE', {
        file: header,
        type,
      });
    } catch (errors) {
      context.commit('ADD_FILE_PARSE_ERRORS', {
        errors,
        type,
      });
    }
  },
};

export default actions;
