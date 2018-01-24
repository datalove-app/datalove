/* global __DEBUG_HOST__, __DEBUG_PORT__, module, window */

import { setup } from '@cycle/most-run';
import { makeHTTPDriver } from '@cycle/http';
// import { timeDriver } from '@cycle/time/most';
import { rerunner, restartable } from 'cycle-restart';
import { wrapStore } from '@sunny-g/react-chrome-redux';
import { applyMiddleware, createStore } from 'redux';
import { composeWithDevTools } from 'remote-redux-devtools';
import { createCycleMiddleware } from '@sunny-g/redux-cycles';

import App from './App';

const cycleMiddleware = createCycleMiddleware();
const { makeActionDriver, makeStateDriver } = cycleMiddleware;

const composeEnhancers = composeWithDevTools({
  realtime: true,
  hostname: __DEBUG_HOST__,
  port: __DEBUG_PORT__,
});
const store = createStore(
  (state = 1) => state,
  composeEnhancers(
    applyMiddleware(cycleMiddleware)
  ),
);
wrapStore(store, { portName: 'MY_APP' }); // make sure portName matches

const makeDrivers = () => ({
  ACTION: restartable(makeActionDriver()),
  STORE: restartable(makeStateDriver()),
  HTTP: restartable(makeHTTPDriver()),
  // Time: timeDriver,
});

const rerun = rerunner(setup, makeDrivers);

function run(Main) {
  console.log('running background script');
  rerun(Main);
}

run(App);

// if (module.hot) {
//   module.hot.accept('./App', () => {
//     console.log('reloading background');

//     // eslint-disable-next-line global-require
//     const newApp = require('./App').default;
//     run(newApp);
//   });
// }
