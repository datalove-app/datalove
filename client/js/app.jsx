App = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function() {
    return {
      title: 'chatter'
    };
  },

  render: function() {
    return (
      <div id="main">
        <RaisedButton label="things" primary={true}/>
      </div>
    )
  }

});
