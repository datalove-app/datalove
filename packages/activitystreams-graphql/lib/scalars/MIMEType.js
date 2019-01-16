"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var language_1 = require("graphql/language");
var lite_1 = require("mime/lite");
var utils_1 = require("../utils");
exports.default = {
    schema: utils_1.loadSchema(path.join(__dirname, 'MIMEType.graphqls')),
    resolver: utils_1.createScalar({
        name: 'MIMEType',
        description: '',
        kind: language_1.Kind.STRING,
        isValid: function (value) {
            return typeof value === 'string'
                && lite_1.default.getExtension(value) !== null;
        },
    }),
};
