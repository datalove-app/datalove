import path = require('path');
import { mergeSchemas, loadSchema } from '../utils';

interface ISchemas {
  [key: string]: string,
}

const typeNames = [
  'ActivityType',
  'ActorType',
  'CollectionType',
  'LinkType',
  'ObjectType',
];

export const schemas: ISchemas = typeNames
  .reduce((_schemas, name) => Object.assign(_schemas, {
    [name]: loadSchema(path.join(__dirname, `${name}.graphqls`)),
  }), {});

export default mergeSchemas(
  typeNames.map(name => schemas[name])
);
