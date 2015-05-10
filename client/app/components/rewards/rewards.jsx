Rewards = React.createClass({
  mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {
      selectedUser: null
    };
  },

  handleClick: function(user, event) {
    // open model (like venmo payment screen)
    // on buy button, it triggers submitWFI... function
    this.setState({selectedUser: user});
    $('#giftModal').openModal();
  },

  render: function() {
    // ?? Meteor.users vs Users
    var users = neoDB.Users.get().user;
    var selectedUser = this.state ? this.state.selectedUser : null;

    return (
      <div>
        <ul className="users collection">
          {users.map(function(user) {
            return (
              <UserListItem
                key={user._id}
                data={user}
                clickHandler={this.handleClick.bind(this, user)}
              />
            );
          }, this)}
        </ul>

        <UserRewardsModal data={selectedUser}/>
      </div>
    );
  }
});