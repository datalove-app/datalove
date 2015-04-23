TransactionListItem = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {
    return {};
  },

  render: function () {
    var limitClass = this.props.data.limitChange < 0 ? 'limit nix' : 'limit gift';

    return (
      <li className="collection-item avatar">
        <img src="https://secure.gravatar.com/avatar/00000000000000000000000000000000?d=mm&f=y" alt="" className="circle"/>
        <span className="title">
          {this.props.data.sourceUsername}
        </span>
        <div className="message">
          {this.props.data.message}
        </div>
        <span className={limitClass}>
          {this.props.data.limitChange}
        </span>
      </li>
    );
  }
});