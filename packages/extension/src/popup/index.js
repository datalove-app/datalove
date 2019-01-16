"use strict";
exports.__esModule = true;
var React = require("react");
var RX = require("reactxp");
var react_redux_1 = require("react-redux");
var react_chrome_redux_1 = require("@sunny-g/react-chrome-redux");
var App_1 = require("./App");
var store = new react_chrome_redux_1.Store({
    portName: 'MY_APP'
});
function run(MainApp) {
    // console.log('store ready', React, RX);
    RX.App.initialize(true, true);
    RX.UserInterface.setMainView(React.createElement(react_redux_1.Provider, { store: store },
        React.createElement(MainApp, null)));
}
// wait for the store to connect to the background page
store
    .ready()
    .then(function () { return run(App_1["default"]); });
// if (module.hot) {
//   module.hot.accept('./App', () => {
//     console.log('reloading popup');
//     // eslint-disable-next-line global-require
//     const newApp = require('./App').default;
//     run(newApp);
//   });
// }
//# sourceMappingURL=index.js.map