Auth = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function() {

  },

  render: function() {
    return (
      <div className="auth container">
        <SignIn />
        <SignUp />
      </div>
    );
  }
});