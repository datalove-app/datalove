"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var enums_1 = require("./enums");
var index_1 = require("./scalars/index");
exports.scalarResolvers = index_1.resolvers;
var index_2 = require("./unions/index");
var index_3 = require("./core/index");
var utils_1 = require("./utils");
exports.default = utils_1.mergeSchemas([enums_1.default, index_1.default, index_2.default, index_3.default]);
