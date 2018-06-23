import path = require('path');
import { loadSchema } from '../utils';

export default loadSchema(path.join(__dirname, 'schema.graphqls'));
