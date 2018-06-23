import fs = require('fs');
import { GraphQLScalarType } from 'graphql';
import { GraphQLError } from 'graphql/error';

interface ICreateScalarOptions {
  kind: string,
  name: string,
  description: string,
  serialize?: (a: any) => any,
  parse?: (a: any) => any,
  isValid?: (a: any) => boolean,
}

const IDENTITY = (x: any) => x;
const createValidator = (isValid: (a: any) => boolean, ERROR: string) =>
  (value: any) => {
    if (!isValid(value)) {
      throw new TypeError(`${ERROR}: ${value}`);
    }
  };

export const mergeSchemas = (schemas: string[]) => schemas
  .reduce((combined, schema) => combined.concat('\n', schema), '');

export const loadSchema = (path: string) =>
  fs.readFileSync(path, 'utf8');

export const createScalar = (options: ICreateScalarOptions): GraphQLScalarType => {
  const KIND = options.kind;
  const TYPE_ERROR = `Can only validate a ${KIND} as a ${options.name}`;
  const VALIDATION_ERROR = `Value is not a valid ${options.name}`;

  const serialize = options.serialize || IDENTITY;
  const parse = options.parse || IDENTITY;
  const isValid = options.isValid || IDENTITY;
  const validate = createValidator(isValid, VALIDATION_ERROR);

  return new GraphQLScalarType({
    name: options.name,
    description: options.description,

    serialize(value) {
      validate(value);
      return serialize(value);
    },

    parseValue(value) {
      const parsed = parse(value);
      validate(parsed);
      return parsed;
    },

    parseLiteral(ast) {
      if (ast.kind !== KIND) {
        throw new GraphQLError(`${TYPE_ERROR}, got a: ${ast.kind}`);
      }

      const parsed = parse((ast as any).value);
      validate(parsed);
      return parsed;
    },
  });
};
