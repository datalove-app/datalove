"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var DateTime_1 = require("./DateTime");
var Duration_1 = require("./Duration");
var LanguageTag_1 = require("./LanguageTag");
var MIMEType_1 = require("./MIMEType");
var NonNegativeInt_1 = require("./NonNegativeInt");
var URI_1 = require("./URI");
var utils_1 = require("../utils");
exports.resolvers = {
    DateTime: DateTime_1.default.resolver,
    Duration: Duration_1.default.resolver,
    LanguageTag: LanguageTag_1.default.resolver,
    MIMEType: MIMEType_1.default.resolver,
    NonNegativeInt: NonNegativeInt_1.default.resolver,
    URI: URI_1.default.resolver,
};
exports.default = utils_1.mergeSchemas([
    DateTime_1.default.schema,
    Duration_1.default.schema,
    LanguageTag_1.default.schema,
    MIMEType_1.default.schema,
    NonNegativeInt_1.default.schema,
    URI_1.default.schema,
]);
