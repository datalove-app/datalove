Router.route('/', function() {
  Router.go('/wall');
});

Router.route('/wall', function() {
  React.render(<Wall />, document.getElementById('main'))
})

Router.route('/market', function() {
  React.render(<YourMarket />, document.getElementById('main'));
});

Router.route('/rewards', function() {
  React.render(<Rewards />, document.getElementById('main'));
});

Router.route('/shopping', function() {
  React.render(<Shopping />, document.getElementById('main'));
});

Router.onBeforeAction(function() {
  if (!Meteor.userId()) {
    React.unmountComponentAtNode(document.getElementById('header'));
    React.unmountComponentAtNode(document.getElementById('footer'));
    React.render(<Auth />, document.getElementById('main'));
    return;
  }

  React.render(<BottomBar />, document.getElementById('footer'));
  this.next();
});