/* eslint-disable no-param-reassign */
const mutations = {
  ADD_FILE_PARSE_ERRORS: (state, { errors, type }) => {
    if (type === 'viewer' || type === 'ass3') {
      state[type].fileParseSuccessful = false;
      state[type].fileParseErrors = errors;
      state[type].file = undefined;
    } else if (type === 'ass1') {
      state.ass1.fileObj.fileParseSuccessful = false;
      state.ass1.fileObj.fileParseErrors = errors;
      state.ass1.fileObj.file = undefined;

      state.ass1.type = undefined;
      state.ass1.csvFileUrl = undefined;
      state.ass1.asciiFileUrl = undefined;
      state.ass1.isAsciiModalActive = false;
      state.ass1.grayImgValues = undefined;
      state.ass1.grayImgValuesReady = false;
      state.ass1.imgWidth = 256;
    } else if (type === 'ass2') {
      state.ass2.fileParseSuccessful = false;
      state.ass2.fileParseErrors = errors;
    }
  },
  ADD_FILE: (state, { file, type }) => {
    if (type === 'viewer' || type === 'ass3') {
      state[type].fileParseSuccessful = true;
      state[type].fileParseErrors = undefined;
      state[type].file = file;
    } else if (type === 'ass1') {
      state.ass1.fileObj.fileParseSuccessful = true;
      state.ass1.fileObj.fileParseErrors = undefined;
      state.ass1.fileObj.file = file;

      state.ass1.type = undefined;
      state.ass1.csvFileUrl = undefined;
      state.ass1.asciiFileUrl = undefined;
      state.ass1.isAsciiModalActive = false;
      state.ass1.grayImgValues = undefined;
      state.ass1.grayImgValuesReady = false;
      state.ass1.imgWidth = 256;
    } else if (type === 'ass2') {
      state.ass2.fileParseSuccessful = true;
      state.ass2.fileParseErrors = undefined;

      state.ass2.partA = { file: file.A };
      state.ass2.partB = { file: file.B };
      state.ass2.partC = { file: file.C };

      state.ass2.partA.fileParseSuccessful = true;
      state.ass2.partA.fileParseErrors = undefined;
      state.ass2.partB.fileParseSuccessful = true;
      state.ass2.partB.fileParseErrors = undefined;
      state.ass2.partC.fileParseSuccessful = true;
      state.ass2.partC.fileParseErrors = undefined;
    }
  },
  ADD_CSV_FILE: (state, file) => {
    state.ass1.csvFileUrl = file;
  },
  ADD_ASCII_ART_FILE: (state, file) => {
    state.ass1.asciiFileUrl = file;
  },
  TOGGLE_ASCII_MODAL_ON: (state) => {
    state.ass1.isAsciiModalActive = true;
  },
  TOGGLE_ASCII_MODAL_OFF: (state) => {
    state.ass1.isAsciiModalActive = false;
  },
  ADD_GRAY_IMG_VALUES: (state, values) => {
    state.ass1.grayImgValues = values;
    state.ass1.grayImgValuesReady = true;
  },
};

export default mutations;
