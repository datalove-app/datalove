import path from 'path';
import { SchemaDirectiveVisitor } from 'graphql-tools';
import { loadSchema } from '../../util/schema';

export class FileDirective extends SchemaDirectiveVisitor {}

export default {
  schema: loadSchema(path.join(__dirname, 'schema.graphqls')),
  schemaDirective: FileDirective,
};
