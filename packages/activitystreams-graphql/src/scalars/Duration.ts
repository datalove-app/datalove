import path = require('path');
import { Kind } from 'graphql/language';
import { pattern } from 'iso8601-duration';
import { createScalar, loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'Duration.graphqls')),
  resolver: createScalar({
    name: 'Duration',
    description: 'ISO 8601 Duration',
    kind: Kind.STRING,
    isValid(value) {
      return typeof value === 'string' && pattern.test(value);
    },
  }),
};
