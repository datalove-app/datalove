UserListItem = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {
    return {};
  },

  render: function () {
    return (
      <li
        className="collection-item avatar"
        onClick={this.props.clickHandler}>
        <img src="https://secure.gravatar.com/avatar/00000000000000000000000000000000?d=mm&f=y" alt="" className="circle"/>
        <span className="title">
          {this.props.data.username}
        </span>
      </li>
    );
  }
});