/* eslint-disable no-param-reassign */
const mutations = {
  ADD_FILE_PARSE_ERRORS: (state, { errors, type }) => {
    if (type === 'viewer') {
      state.viewer.fileParseSuccessful = false;
      state.viewer.fileParseErrors = errors;
      state.viewer.file = undefined;
    } else if (type === 'ass1') {
      state.ass1.fileObj.fileParseSuccessful = true;
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
      state.ass2.file = undefined;
    }
  },
  ADD_FILE: (state, { file, type }) => {
    if (type === 'viewer') {
      state.viewer.fileParseSuccessful = true;
      state.viewer.fileParseErrors = undefined;
      state.viewer.file = file;
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
      state.ass2.file = file;
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
