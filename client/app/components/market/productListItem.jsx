ProductListItem = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function () {

  },

  render: function () {
    return (
      <li>
        {this.props.data}
      </li>
    );
  }
});