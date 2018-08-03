"use strict";
exports.__esModule = true;
var readerT = require("fp-ts/lib/ReaderT");
var Callbag_1 = require("../util/Callbag");
var readerTCallbag = readerT.getReaderT(Callbag_1.callbag);
exports.URI = 'ReaderCallbag';
var ReaderCallbag = /** @class */ (function () {
    function ReaderCallbag(run) {
        this.run = run;
    }
    ReaderCallbag.prototype.map = function (f) {
        return new ReaderCallbag(readerTCallbag.map(f, this.run));
    };
    ReaderCallbag.prototype.of = function (b) {
        return of(b);
    };
    ReaderCallbag.prototype.ap = function (fab) {
        return new ReaderCallbag(readerTCallbag.ap(fab.run, this.run));
    };
    ReaderCallbag.prototype.chain = function (f) {
        return new ReaderCallbag(readerTCallbag.chain(function (a) { return f(a).run; }, this.run));
    };
    return ReaderCallbag;
}());
exports.ReaderCallbag = ReaderCallbag;
var map = function (fa, f) {
    return fa.map(f);
};
var of = function (a) {
    return new ReaderCallbag(readerTCallbag.of(a));
};
var ap = function (fab, fa) {
    return fa.ap(fab);
};
var chain = function (fa, f) {
    return fa.chain(f);
};
exports.readerTFromReader = readerT.fromReader(Callbag_1.callbag);
exports.fromReader = function (fa) {
    return new ReaderCallbag(exports.readerTFromReader(fa));
};
exports.readerCallbag = {
    URI: exports.URI,
    map: map,
    of: of,
    ap: ap,
    chain: chain
};
//# sourceMappingURL=ReaderCallbag.js.map