const path = require('path');

module.exports = [
  {
    entry: './dist/script.js',
    mode: 'production',
    output: {
      filename: 'bundle.js',
      path: path.resolve(__dirname, 'dist'),
      libraryTarget: 'var',
      library: "EntryPoint",
    }
  }
];

