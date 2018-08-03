import path = require('path');
import { createScalar } from 'activitystreams-graphql/src/utils';
import { Kind } from 'graphql/language';
import { loadSchema } from '../util/schema';

export default {
  schema: loadSchema(path.join(__dirname, 'Buffer.graphqls')),
  resolver: createScalar({
    name: 'Buffer',
    description: 'Raw bytes',
    kind: Kind.STRING,
    isValid(value) {
      return false;
    },
  }),
};
