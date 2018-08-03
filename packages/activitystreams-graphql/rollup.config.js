import typescript from 'rollup-plugin-typescript';

export default {
  entry: './src/main.ts',
  output: './lib/main.js',
  format: 'cjs',
  plugins: [
    typescript(),
  ],
};
