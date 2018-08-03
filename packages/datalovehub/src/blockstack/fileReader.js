"use strict";
exports.__esModule = true;
var ReaderCallbag_1 = require("./ReaderCallbag");
function getFileReader(path, options) {
    return new ReaderCallbag_1.ReaderCallbag(function (blockstack) {
        return function getFileSource(start, sink) {
            if (start !== 0)
                return;
            sink(0, getFileSource);
            blockstack.getFile(path, options)
                .then(function (file) { return sink(1, file); }, function (err) { return sink(2, err); });
        };
    });
}
exports.getFileReader = getFileReader;
function putFileReader(path, content, options) {
    return new ReaderCallbag_1.ReaderCallbag(function (blockstack) {
        return function putFileSource(start, sink) {
            if (start === 0)
                return;
            sink(0, putFileSource);
            blockstack.putFile(path, content, options)
                .then(function (res) { return sink(1, res); }, function (err) { return sink(2, err); });
        };
    });
}
exports.putFileReader = putFileReader;
//# sourceMappingURL=reader.js.map