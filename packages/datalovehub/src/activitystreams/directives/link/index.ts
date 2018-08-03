import path = require('path');
import { SchemaDirectiveVisitor } from 'graphql-tools';
import { loadSchema } from '../../util/schema';

export class LinkDirective extends SchemaDirectiveVisitor {}

export default {
  schema: loadSchema(path.join(__dirname, 'schema.graphqls')),
  schemaDirective: LinkDirective,
};
