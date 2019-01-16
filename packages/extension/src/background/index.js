"use strict";
/* global __DEBUG_HOST__, __DEBUG_PORT__, module, window */
exports.__esModule = true;
var most_run_1 = require("@cycle/most-run");
var http_1 = require("@cycle/http");
// import { timeDriver } from '@cycle/time/most';
var react_chrome_redux_1 = require("@sunny-g/react-chrome-redux");
var redux_cycles_1 = require("@sunny-g/redux-cycles");
var cycle_restart_1 = require("cycle-restart");
var redux_1 = require("redux");
var remote_redux_devtools_1 = require("remote-redux-devtools");
var App_1 = require("./App");
var cycleMiddleware = redux_cycles_1.createCycleMiddleware();
var makeActionDriver = cycleMiddleware.makeActionDriver, makeStateDriver = cycleMiddleware.makeStateDriver;
var composeEnhancers = remote_redux_devtools_1.composeWithDevTools({
    realtime: true,
    hostname: __DEBUG_HOST__,
    port: __DEBUG_PORT__
});
var store = redux_1.createStore(function (state) {
    if (state === void 0) { state = 1; }
    return state;
}, composeEnhancers(redux_1.applyMiddleware(cycleMiddleware)));
react_chrome_redux_1.wrapStore(store, { portName: 'MY_APP' }); // make sure portName matches
var makeDrivers = function () { return ({
    ACTION: cycle_restart_1.restartable(makeActionDriver()),
    STATE: cycle_restart_1.restartable(makeStateDriver()),
    HTTP: cycle_restart_1.restartable(http_1.makeHTTPDriver())
}); };
var rerun = cycle_restart_1.rerunner(most_run_1.setup, makeDrivers);
function run(Main) {
    console.log('running background script');
    rerun(Main);
}
run(App_1["default"]);
// if (module.hot) {
//   module.hot.accept('./App', () => {
//     console.log('reloading background');
//     // eslint-disable-next-line global-require
//     const newApp = require('./App').default;
//     run(newApp);
//   });
// }
//# sourceMappingURL=index.js.map