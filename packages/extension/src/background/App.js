"use strict";
exports.__esModule = true;
function main(sources) {
    var pong$ = sources.ACTION
        .filter(function (action) { return action.type === 'PING'; })
        .constant({ type: 'PONG' });
    return {
        ACTION: pong$
    };
}
exports["default"] = main;
//# sourceMappingURL=App.js.map