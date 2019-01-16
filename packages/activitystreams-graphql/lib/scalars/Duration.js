"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var language_1 = require("graphql/language");
var iso8601_duration_1 = require("iso8601-duration");
var utils_1 = require("../utils");
exports.default = {
    schema: utils_1.loadSchema(path.join(__dirname, 'Duration.graphqls')),
    resolver: utils_1.createScalar({
        name: 'Duration',
        description: 'ISO 8601 Duration',
        kind: language_1.Kind.STRING,
        isValid: function (value) {
            return typeof value === 'string' && iso8601_duration_1.pattern.test(value);
        },
    }),
};
