"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var language_1 = require("graphql/language");
var bcp47_validate_1 = require("bcp47-validate");
var utils_1 = require("../utils");
exports.default = {
    schema: utils_1.loadSchema(path.join(__dirname, 'LanguageTag.graphqls')),
    resolver: utils_1.createScalar({
        name: 'LanguageTag',
        description: 'BCP 47 Language Tag',
        kind: language_1.Kind.STRING,
        isValid: function (value) {
            return typeof value === 'string' && bcp47_validate_1.default(value);
        },
    }),
};
