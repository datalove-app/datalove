const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');
const webpack = require('webpack');

const BASE_CONFIG = {
  mode: 'none',
  stats: 'verbose',

  node: {
    console: false,
    global: false,
    process: false,
    setImmediate: false,
    __dirname: false,
    __filename: false,
  },

  resolve: {
    extensions: ['.ts'],
  },

  module: {
    rules: [
      {
        test: /\.ts/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: [
                '@babel/preset-env',
                '@babel/preset-typescript',
              ],
            },
          },
        ],
      },
    ],
  },

  plugins: [
    new webpack.DefinePlugin({
      // options are 'OFF', 'ERROR', 'WARN', 'INFO' and 'DEBUG'
      LOG_LEVEL: process.env.NODE_ENV === 'production'
        ? JSON.stringify('OFF')
        : JSON.stringify('DEBUG'),
    }),
  ],
};

module.exports.BASE_CONFIG = BASE_CONFIG;
module.exports.makeWebpackConfig = (opts) => {
  const { dnaDir, srcDir, zomes: zomeNames } = opts;

  const entries = zomeNames
    .reduce((allEntries, name) => ({
      ...allEntries,
      [name]: path.resolve(srcDir, 'zomes', name, 'index.ts'),
    }), {});

  const outputs = zomeNames
    .reduce((allOutputs, name) => ({
      ...allOutputs,
      [name]: {
        path: path.resolve(dnaDir, name),
        filename: `${name}.js`,
        libraryTarget: 'this',
      },
    }), {});

  const basePlugins = [
    new CopyWebpackPlugin([
      { from: 'dna.json', to: dnaDir },
      { from: 'properties_schema.json', to: dnaDir },
    ], {
      context: srcDir,
    }),
  ];

  return zomeNames
    .map(name => Object.assign({}, BASE_CONFIG, {
      name,
      entry: entries[name],
      output: outputs[name],
      plugins: BASE_CONFIG.plugins.concat(basePlugins),
    }));
};
