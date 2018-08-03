import fs from 'fs';

export const mergeSchemas = (schemas: string[]) => schemas
  .reduce((combined, schema) => combined.concat('\n', schema), '');

export const loadSchema = (path: string) =>
  fs.readFileSync(path, 'utf8');
