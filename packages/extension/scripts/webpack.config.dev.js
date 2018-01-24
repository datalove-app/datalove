const path = require('path');
const webpack = require('webpack');
const ChromeExtensionReloaderPlugin = require('webpack-chrome-extension-reloader');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const isDev = process.env.NODE_ENV === 'development';
const isProd = process.env.NODE_ENV === 'production';
const customPublicPath = path.join(__dirname, 'customPublicPath');
const hotScript = 'webpack-hot-middleware/client?path=__webpack_hmr?dynamicPublicPath=true';

const REPO_DIR = path.resolve(__dirname, '../../../');
const ROOT_DIR = path.resolve(__dirname, '../');
const SRC_DIR = path.resolve(ROOT_DIR, 'src');
const DIST_DIR = path.resolve(ROOT_DIR, 'dist');
const REPO_MODULES = path.resolve(REPO_DIR, 'node_modules');
const ROOT_MODULES = path.resolve(ROOT_DIR, 'node_modules');

const HOST = 'localhost';
const PORT = 8080;
const DEBUG_HOST = 'localhost';
const DEBUG_PORT = 8000;
const REGEX_JSXTSX = /\.(tsx?)|(jsx?)/;
const REGEX_JSX = /\.(jsx?)/;
const REGEX_TSX = /\.(tsx?)/;

const polyfills = [
  '@babel/polyfill',
  'papp-polyfill',
  // customPublicPath,
  // hotScript,
  // 'react-hot-loader/patch',
];

const entries = {
  background: polyfills
    .concat([
      path.resolve(SRC_DIR, 'background', 'index.ts')
    ]),
  content: polyfills
    .concat([
      path.resolve(SRC_DIR, 'content', 'index.ts')
    ]),
  popup: polyfills
    .concat([
      path.resolve(SRC_DIR, 'popup', 'index.tsx')
    ]),
};

const output = {
  path: DIST_DIR,
  filename: '[name].js',
};

const aliases = {
  // 'react-chrome-redux': path.resolve(ROOT_MODULES, 'react-chrome-redux'),
  // 'redux-cycles': path.resolve(REPO_MODULES, 'redux-cycles'),
};
const extensions = ['.js', '.jsx', '.ts', '.tsx'];
const modules = [
  REPO_MODULES,
  ROOT_MODULES,
];

const sourceMapRules = {
  test: REGEX_JSXTSX,
  use: 'source-map-loader',
  enforce: 'pre',
  include: [
    SRC_DIR,
  ],
};
const linterRules = {
  test: REGEX_JSXTSX,
  use: 'eslint-loader',
  enforce: 'pre',
  include: [
    SRC_DIR,
  ],
};
const srcRules = {
  test: REGEX_TSX,
  use: [
    { loader: 'react-hot-loader/webpack' },
    { loader: 'babel-loader',
      options: {
        presets: [
          ['@babel/preset-env', { modules: false }],
          '@babel/preset-react',
          '@babel/preset-stage-0',
        ],
      },
    },
    { loader: 'ts-loader' },
    { loader: 'babel-loader',
      options: {
        plugins: [
          '@babel/plugin-syntax-jsx',
        ],
        presets: [
          'proposal-typescript',
        ],
      },
    },
  ],
};
const fileRules = {
  test: /\.(ico|eot|otf|webp|ttf|woff|woff2)(\?.*)?$/,
  use: {
    loader: 'file-loader',
    options: {
      limit: 100000,
    },
  },
};

const config = {
  devtool: 'source-map',
  // devServer: {
  //   contentBase: DIST_DIR,
  //   port: PORT,
  //   hot: true,
  //   quiet: false,
  //   noInfo: false,
  //   stats: {
  //     assets: false,
  //     children: false,
  //     chunkModules: false,
  //     chunks: false,
  //     colors: true,
  //     hash: false,
  //     timings: false,
  //     version: false,
  //   },
  // },

  entry: entries,

  output: output,

  resolve: {
    alias: aliases,
    extensions: extensions,
    modules: modules,
  },

  module: {
    rules: [
      sourceMapRules,
      linterRules,
      srcRules,
      fileRules,
    ],
  },

  plugins: [
    new webpack.DefinePlugin({
      '__HOST__': JSON.stringify(HOST),
      '__PORT__': PORT,
      '__DEBUG_HOST__': JSON.stringify(DEBUG_HOST),
      '__DEBUG_PORT__': DEBUG_PORT,
      'process.env': {
        NODE_ENV: JSON.stringify('development'),
      },
    }),
    new ChromeExtensionReloaderPlugin({
      port: PORT,
      entries: {
        background: 'background',
        contentScript: 'content',
      },
    }),
    // new webpack.NamedModulesPlugin(),
    // new webpack.HotModuleReplacementPlugin(),
    // new HtmlWebpackPlugin({
    //   inject: true,
    //   chunks: ['index'],
    //   filename: 'index.html',
    //   template: path.resolve(SRC_DIR, 'index.html'),
    // }),
  ],
};

module.exports = config;
