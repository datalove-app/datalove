import path = require('path');
import { Kind } from 'graphql/language';
import mime from 'mime/lite';
import { createScalar, loadSchema } from '../utils';

export default {
  schema: loadSchema(path.join(__dirname, 'MIMEType.graphqls')),
  resolver: createScalar({
    name: 'MIMEType',
    description: '',
    kind: Kind.STRING,
    isValid(value) {
      return typeof value === 'string'
        && mime.getExtension(value) !== null;
    },
  }),
};
