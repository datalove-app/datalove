import path = require('path');
import { SchemaDirectiveVisitor } from 'graphql-tools';
import { loadSchema, mergeSchemas } from '../../util/schema';

export class CollectionDirective extends SchemaDirectiveVisitor {}

export default {
  schema: mergeSchemas([
    loadSchema(path.join(__dirname, 'enums.graphqls')),
    loadSchema(path.join(__dirname, 'schema.graphqls')),
  ]),
  schemaDirective: CollectionDirective,
};
