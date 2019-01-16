"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var utils_1 = require("../utils");
exports.default = {
    schema: utils_1.loadSchema(path.join(__dirname, 'LinkRelation.graphqls')),
    resolver: null,
};
