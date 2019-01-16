import * as React from 'react';
import * as RX from 'reactxp';
import { Provider } from 'react-redux';
import { Store } from '@sunny-g/react-chrome-redux';
import App from './App';

const store = new Store({
  portName: 'MY_APP', // communication port name
});

function run(MainApp: any) {  // eslint-disable-line
  // console.log('store ready', React, RX);

  RX.App.initialize(true, true);
  RX.UserInterface.setMainView(
    <Provider store={store}>
      <MainApp />
    </Provider>
  );
}

// wait for the store to connect to the background page
store
  .ready()
  .then(() => run(App));

// if (module.hot) {
//   module.hot.accept('./App', () => {
//     console.log('reloading popup');

//     // eslint-disable-next-line global-require
//     const newApp = require('./App').default;
//     run(newApp);
//   });
// }
