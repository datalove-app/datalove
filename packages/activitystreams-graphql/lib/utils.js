"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
var graphql_1 = require("graphql");
var error_1 = require("graphql/error");
var IDENTITY = function (x) { return x; };
var createValidator = function (isValid, ERROR) {
    return function (value) {
        if (!isValid(value)) {
            throw new TypeError(ERROR + ": " + value);
        }
    };
};
exports.mergeSchemas = function (schemas) { return schemas
    .reduce(function (combined, schema) { return combined.concat('\n', schema); }, ''); };
exports.loadSchema = function (path) {
    return fs.readFileSync(path, 'utf8');
};
exports.createScalar = function (options) {
    var KIND = options.kind;
    var TYPE_ERROR = "Can only validate a " + KIND + " as a " + options.name;
    var VALIDATION_ERROR = "Value is not a valid " + options.name;
    var serialize = options.serialize || IDENTITY;
    var parse = options.parse || IDENTITY;
    var isValid = options.isValid || IDENTITY;
    var validate = createValidator(isValid, VALIDATION_ERROR);
    return new graphql_1.GraphQLScalarType({
        name: options.name,
        description: options.description,
        serialize: function (value) {
            validate(value);
            return serialize(value);
        },
        parseValue: function (value) {
            var parsed = parse(value);
            validate(parsed);
            return parsed;
        },
        parseLiteral: function (ast) {
            if (ast.kind !== KIND) {
                throw new error_1.GraphQLError(TYPE_ERROR + ", got a: " + ast.kind);
            }
            var parsed = parse(ast.value);
            validate(parsed);
            return parsed;
        },
    });
};
