import path = require('path');
import { Kind } from 'graphql/language';
import validURL from 'valid-url';
import { createScalar, loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'URI.graphqls')),
  resolver: createScalar({
    name: 'URI',
    description: 'RFC3986 URI',
    kind: Kind.STRING,
    isValid(value) {
      return typeof value === 'string' && validURL.isUri(value);
    },
  }),
};
