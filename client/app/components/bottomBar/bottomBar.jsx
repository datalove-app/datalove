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
            <a href="/wall" className="bottom-links">Wall</a>
            {/* <a href="/market" className="bottom-links">Your Market</a> */}
            <a href="/rewards" className="bottom-links">Rewards</a>
            {/* <a href="/shopping" className="bottom-links">Shopping</a> */}
            <SignOut />
          </div>
        </div>
      </footer>
    );
  }
});