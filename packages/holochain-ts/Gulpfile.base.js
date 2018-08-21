const fs = require('fs');
const path = require('path');
const TJS = require('typescript-json-schema');
const webpack = require('webpack');

const writeSchemaToDist = (schema, filePath) => new Promise((resolve) => {
  fs.writeFile(filePath, JSON.stringify(schema, null, 2), (err) => {
    if (err) { console.error(`Failure writing ${filePath} to disk`); }
    return resolve();
  });
});

const mkdirp = (dir, errMessage) => new Promise((resolve) => {
  fs.mkdir(dir, (err) => {
    if (err) { console.error(errMessage); }
    return resolve();
  });
});

// ************************************************ //
// Tasks and generic configuration
// ************************************************ //

const TASK_NAMES = {
  default: 'default',
  initDNA: 'init:dna',
  buildDNA: 'build:dna',
  buildJSONSchemas: 'build:json_schemas',
};

const tasks = {
  initDNA: (gulp, opts) => gulp.task(TASK_NAMES.initDNA, () => {
    const dnaDir = mkdirp(opts.dnaDir, 'DNA directory already exists, moving on...');
    return Promise
      .all([dnaDir].concat(opts.zomes.map((name) => {
        const ZOME_DIR = path.resolve(opts.dnaDir, name);
        return mkdirp(ZOME_DIR, `Zome \`${name}\` directory already exists, moving on...`);
      })));
  }),

  buildDNA: (gulp, makeWebpackConfig, opts) => {
    gulp.task(TASK_NAMES.buildDNA, [TASK_NAMES.initDNA], (done) => {
      const webpackRunner = webpack(makeWebpackConfig(opts));
      webpackRunner.run(done);
    });
  },

  buildJSONSchemas: (gulp, opts) => {
    gulp.task(TASK_NAMES.buildJSONSchemas, [TASK_NAMES.initDNA], () => {
      const settings = {
        ignoreErrors: true,
        required: true,
      };
      const program = TJS.programFromConfig(opts.tsConfig);
      const generator = TJS.buildGenerator(program, settings);

      const schemaGenerators = Object
        .keys(opts.interfaces)
        .map((interfaceName) => {
          const schema = generator.getSchemaForSymbol(interfaceName);
          const outFilePath = opts.interfaces[interfaceName];
          return writeSchemaToDist(schema, outFilePath);
        });

      return Promise.all(schemaGenerators);
    });
  },

  default: gulp => gulp.task(TASK_NAMES.default, [
    TASK_NAMES.initDNA,
    TASK_NAMES.buildDNA,
    TASK_NAMES.buildJSONSchemas,
  ]),
};

module.exports = {
  TASK_NAMES,
  tasks,
};
