const state = {
  viewer: {
    fileParseErrors: undefined,
    file: undefined,
    fileParseSuccessful: false,
  },
  ass1: {
    fileObj: {
      fileParseErrors: undefined,
      file: undefined,
      fileParseSuccessful: false,
    },
    type: undefined,
    csvFileUrl: undefined,
    asciiFileUrl: undefined,
    isAsciiModalActive: false,
    grayImgValues: undefined,
    grayImgValuesReady: false,
    imgWidth: 256,
  },
  ass2: {
    fileParseErrors: undefined,
    fileParseSuccessful: false,
    partA: undefined,
    partB: undefined,
    partC: undefined,
  },
  ass3: {
    fileParseErrors: undefined,
    file: undefined,
    fileParseSuccessful: false,
    fileText: undefined,
    kernel: undefined,
  },
  home: {
    fileParseErrors: undefined,
    file: undefined,
    fileParseSuccessful: false,
  },
  page: ['viewer', 'ass1', 'home', 'ass2','ass3'],
  pageName: {
    viewer: 'Viewer',
    ass1: 'Assignment 1',
    ass2: 'Assignment 2',
    ass3: 'Assignment 3',
    home: 'Home',
  },
  pageNameRev: {
    'Assignment 1': 'ass1',
    'Assignment 2': 'ass2',
    'Assignment 3': 'ass3',
    Viewer: 'viewer',
    Home: 'home',
  },
};

export default state;