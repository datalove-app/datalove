import path = require('path');
import { NonNegativeInt } from '@okgrow/graphql-scalars';
import { loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'NonNegativeInt.graphqls')),
  resolver: NonNegativeInt,
};
