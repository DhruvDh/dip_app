/* eslint-disable no-param-reassign */
const mutations = {
  ADD_FILE_PARSE_ERRORS: (state, { errors, type }) => {
    state[type].fileParseSuccessful = false;
    state[type].fileParseErrors = errors;
    state[type].file = undefined;

    if (type === 'ass1') {
      state.ass1.type = undefined;
      state.ass1.csvFileUrl = undefined;
      state.ass1.asciiFileUrl = undefined;
      state.ass1.isAsciiModalActive = false;
      state.ass1.grayImgValues = undefined;
      state.ass1.grayImgValuesReady = false;
      state.ass1.imgWidth = 256;
    }
  },
  ADD_FILE: (state, { file, type }) => {
    state[type].fileParseSuccessful = true;
    state[type].fileParseErrors = undefined;
    state[type].file = file;

    if (type === 'ass1') {
      state.ass1.type = undefined;
      state.ass1.csvFileUrl = undefined;
      state.ass1.asciiFileUrl = undefined;
      state.ass1.isAsciiModalActive = false;
      state.ass1.grayImgValues = undefined;
      state.ass1.grayImgValuesReady = false;
      state.ass1.imgWidth = 256;
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
