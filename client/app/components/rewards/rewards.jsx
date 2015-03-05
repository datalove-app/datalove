Rewards = React.createClass({
  mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {};
  },

  render: function() {
    var users = Meteor.users.find();

    return (
      <div>
        <ul className="collection">
          {users.map(function(user) {
            return <UserListItem key={user._id} data={user}/>
          })}
        </ul>
      </div>
    );
  }
});