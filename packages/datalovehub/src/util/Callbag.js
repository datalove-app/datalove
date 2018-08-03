"use strict";
exports.__esModule = true;
var callbag_flat_map_1 = require("callbag-flat-map");
var callbag_map_1 = require("callbag-map");
var callbag_of_1 = require("callbag-of");
var callbag_pipe_1 = require("callbag-pipe");
exports.URI = 'Callbag';
var cap = function (m) { return function (source) { return callbag_pipe_1["default"](source, callbag_flat_map_1["default"](function (e) { return callbag_pipe_1["default"](m, callbag_map_1["default"](function (fn) { return fn(e); })); })); }; };
var map = function (fa, f) {
    return callbag_pipe_1["default"](fa, callbag_map_1["default"](f));
};
var of = function (a) {
    return callbag_of_1["default"](a);
};
var ap = function (fab, fa) {
    return callbag_pipe_1["default"](fa, cap(fab));
};
var chain = function (fa, f) {
    return callbag_pipe_1["default"](fa, callbag_flat_map_1["default"](f));
};
exports.callbag = {
    URI: exports.URI,
    map: map,
    of: of,
    ap: ap,
    chain: chain
};
//# sourceMappingURL=Callbag.js.map