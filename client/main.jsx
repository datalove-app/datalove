Meteor.startup(function () {

  //Needed for React Developer Tools
  window.React = React;

  //Needed for onTouchTap
  //Can go away when react 1.0 release
  //Check this repo:
  //https://github.com/zilverline/react-tap-event-plugin
  //injectTapEventPlugin();

  // Render the main app react component into the document body.
  // For more details see: https://facebook.github.io/react/docs/top-level-api.html#react.render
  React.render(<App />, document.body);

  // Stellar Setup
  Amount = stellar.Amount;
  Remote = stellar.Remote;
  utils = stellar.utils;

  remote = new Remote({
    trusted: false,
    local_signing: true,
    local_fee: true,
    fee_cushion: 1.5,
    max_fee: 15,
    servers: [
      {
        host: stellardCxn.host,
        port: stellardCxn.port,
        secure: stellardCxn.secure
      }
    ]
  });

  remote.connect();

  // sets session vars to Stellar stuff
  setStellarSession();

  // TODO: FIX THIS so it keeps the remote alive;
  (function() {
    console.log('stellar heartbeat');
    Meteor.setInterval(function() {
      // reconnects to stellard
      if (remote.state === 'offline') {
        console.log('reconnecting to stellard');
        remote.connect();
      }

      // guarantees that stellar info stays in Session
      if (!Session.get('myAddr')) {
        setStellarSession();
      }
    }, 15000);
  })();

  // ???
  Tracker.autorun(function() {
    Meteor.subscribe('userData');
  });

  // everytime Meteor.user changes, do something
  Tracker.autorun(function() {
    console.log('autorunning Meteor.user()');
    var user = Meteor.user();
    if (user) {
      Session.set('user', user);
    }
  })
});