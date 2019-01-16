"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var path = require("path");
var utils_1 = require("../utils");
var typeNames = [
    'ActivityType',
    'ActorType',
    'CollectionType',
    'LinkType',
    'ObjectType',
];
exports.schemas = typeNames
    .reduce(function (_schemas, name) {
    var _a;
    return Object.assign(_schemas, (_a = {},
        _a[name] = utils_1.loadSchema(path.join(__dirname, name + ".graphqls")),
        _a));
}, {});
exports.default = utils_1.mergeSchemas(typeNames.map(function (name) { return exports.schemas[name]; }));
