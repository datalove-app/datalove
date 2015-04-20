Shopping = React.createClass({
  mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {};
  },

  handleClick: function(e) {
    // switch route to user's shop
    // trigger maxflow calc
      // use result for styling productListings
    console.log('clicked shopping userListItem');
  },

  render: function() {
    var users = Meteor.users.find();

    return (
      <div>
        <ul className="users collection">
          {users.map(function(user) {
            return <UserListItem
              key={user._id}
              data={user}
              clickHandler={this.handleClick}
            />
          }, this)}
        </ul>
      </div>
    );
  }
});