Router.route('/', function() {
  Router.go('/rewards');
});

Router.route('/market', function() {
  React.render(<Market />, document.getElementById('main'));
  React.render(<BottomBar />, document.getElementById('footer'));
});

Router.route('/rewards', function() {
  React.render(<Rewards />, document.getElementById('main'));
  React.render(<BottomBar />, document.getElementById('footer'));
});

Router.route('/shopping', function() {
  React.render(<Shopping />, document.getElementById('main'));
  React.render(<BottomBar />, document.getElementById('footer'));
});

Router.onBeforeAction(function() {
  if (!Meteor.userId()) {
    React.unmountComponentAtNode(document.getElementById('header'));
    React.unmountComponentAtNode(document.getElementById('footer'));
    React.render(<Auth />, document.getElementById('main'));
    return;
  }

  this.next();
});