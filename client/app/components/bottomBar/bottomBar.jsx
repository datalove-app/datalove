BottomBar = React.createClass({
  mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {};
  },

  render: function() {
    return (
      <footer className="page-footer">
        <div className="container">
          <div className="row">
            <a href="/market" className="">Market</a>
            <a href="/rewards" className="">Rewards</a>
            <a href="/shopping" className="">Shopping</a>
            <SignOut />
          </div>
        </div>
      </footer>
    );
  }
});