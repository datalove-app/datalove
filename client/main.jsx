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

  if (Meteor.userId()) {
    setStellarSession();
  }

  // TODO: does this need to be run on interval?
  remote.connect();
});