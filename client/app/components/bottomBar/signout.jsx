SignOut = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function() {
  },

  signout: function() {
    Meteor.logout(function(err) {
      console.log('signed out', err);
      Router.go('/');
    });
  },

  render: function() {
    return (
      <a href="#" onClick={this.signout}>Sign Out</a>
    );
  }
});