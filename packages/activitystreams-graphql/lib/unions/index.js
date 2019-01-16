"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var utils_1 = require("../utils");
exports.default = utils_1.loadSchema(path.join(__dirname, 'schema.graphqls'));
