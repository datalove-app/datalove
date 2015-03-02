App = React.createClass({
  mixins: [ReactMeteor.Mixin],

  getMeteorState: function() {
    return {
      title: 'chatter'
    };
  },

  handleClick: function(event) {
    event.stopPropagation();
    console.log(event.target);
  },

  render: function() {
    return (
      <div id="bodyWrapper">
        <div id="header"></div>
        <main id="main"></main>
        <div id="footer"></div>
      </div>
    );
  }

});
