import CollectionDirective from './collection';
import FileDirective from './file';
import LinkDirective from './link';
import MentionDirective from './mention';
import ObjectDirective from './object';
import { mergeSchemas } from '../util/schema';

const schemaDirectives = {
  collection: CollectionDirective.schemaDirective,
  file: FileDirective.schemaDirective,
  link: LinkDirective.schemaDirective,
  mention: MentionDirective.schemaDirective,
  object: ObjectDirective.schemaDirective,
};

const schema = mergeSchemas([
  CollectionDirective.schema,
  FileDirective.schema,
  LinkDirective.schema,
  MentionDirective.schema,
  ObjectDirective.schema,
]);

export default {
  schema,
  schemaDirectives,
};
