import path = require('path');
import { Kind } from 'graphql/language';
import validate from 'bcp47-validate';
import { createScalar, loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'LanguageTag.graphqls')),
  resolver: createScalar({
    name: 'LanguageTag',
    description: 'BCP 47 Language Tag',
    kind: Kind.STRING,
    isValid(value) {
      return typeof value === 'string' && validate(value);
    },
  }),
};
