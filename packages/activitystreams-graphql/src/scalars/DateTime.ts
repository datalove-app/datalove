import path = require('path');
import { DateTime } from '@okgrow/graphql-scalars';
import { loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'DateTime.graphqls')),
  resolver: DateTime,
};
