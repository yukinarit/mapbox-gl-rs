const path = require('path');

module.exports = {
    mode: 'development',
    entry: './dist/app.js',
    resolve: {
          extensions: ['.js', '.wasm'],
        },
    output: {
          path: path.resolve(__dirname, 'app'),
    },
    experiments: {
      syncWebAssembly: true,
    },
}
