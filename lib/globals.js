var _ = lodash;

var stellarConnections = {
  local: {
    name: 'local',
    protocol: 'ws://',
    host: 'localhost',
    port: '5006',
    url: 'ws://localhost:5006',
    secure: false
  },

  test: {
    name: 'test',
    protocol: 'wss://',
    host: 'test.stellar.org',
    port: '9001',
    url: 'wss://test.stellar.org:9001',
    secure: true
  },

  live: {
    name: 'live',
    protocol: 'wss://',
    host: 'live.stellar.org',
    port: '9001',
    url: 'wss://live.stellar.org:9001',
    secure: true
  }
};

// what stellard are we connecting to?
stellardCxn = stellarConnections.test;

// get free test stellar from SDF?
getFreeStellar = false;

// stellar constants
base_fee = 200 * 10e6;
