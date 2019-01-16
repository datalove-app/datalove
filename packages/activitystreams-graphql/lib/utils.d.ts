import { GraphQLScalarType } from 'graphql';
interface ICreateScalarOptions {
    kind: string;
    name: string;
    description: string;
    serialize?: (a: any) => any;
    parse?: (a: any) => any;
    isValid?: (a: any) => boolean;
}
export declare const mergeSchemas: (schemas: string[]) => string;
export declare const loadSchema: (path: string) => string;
export declare const createScalar: (options: ICreateScalarOptions) => GraphQLScalarType;
export {};
