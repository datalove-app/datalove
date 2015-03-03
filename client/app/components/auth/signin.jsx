SignIn = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {

  },

  openModal: function() {
    $('#signinModal').openModal();
  },

  closeModal: function() {
    $('#signinModal').closeModal();
  },

  signinUser: function(event) {
    event.preventDefault();

    var username = this.refs.username.getDOMNode().value;
    var passwordNode = this.refs.password.getDOMNode();
    var password = passwordNode.value;

    Meteor.loginWithPassword(username, password, function(err) {
      if (err) {

      } else {
        this.closeModal();
      }
    }.bind(this));
  },

  render: function () {
    return (
      <span className="authButton container">
        {/* renders the modal trigger, a button */}
        <a
          className="authButton btn-large modal-trigger"
          href="#signinModal"
          onClick={this.openModal}>
          SIGNIN
        </a>

        {/* renders the modal */}
        <div id="signinModal" className="modal">
          <div className="modal-content">
            <h4>Sign In</h4>
            <form onSubmit={this.signinUser}>
              <input
                placeholder="username"
                type="text"
                ref="username"
              />
              <input
                placeholder="password"
                type="password"
                ref="password"
              />
              <input
                type="submit"
                className="authSubmit"
              />
            </form>
          </div>
        </div>
      </span>
    );
  }
});