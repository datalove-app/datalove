import path = require('path');
import { loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'LinkRelation.graphqls')),
  resolver: null,
};
