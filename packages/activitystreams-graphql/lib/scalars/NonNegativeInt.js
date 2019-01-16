"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var graphql_scalars_1 = require("@okgrow/graphql-scalars");
var utils_1 = require("../utils");
exports.default = {
    schema: utils_1.loadSchema(path.join(__dirname, 'NonNegativeInt.graphqls')),
    resolver: graphql_scalars_1.NonNegativeInt,
};
