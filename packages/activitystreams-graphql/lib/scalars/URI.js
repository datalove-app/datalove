"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var language_1 = require("graphql/language");
var valid_url_1 = require("valid-url");
var utils_1 = require("../utils");
exports.default = {
    schema: utils_1.loadSchema(path.join(__dirname, 'URI.graphqls')),
    resolver: utils_1.createScalar({
        name: 'URI',
        description: 'RFC3986 URI',
        kind: language_1.Kind.STRING,
        isValid: function (value) {
            return typeof value === 'string' && valid_url_1.default.isUri(value);
        },
    }),
};
