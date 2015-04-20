UserRewardsModal = React.createClass({
	mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {};
  },

  submitTxn: function(event) {
  	event.preventDefault();

  	var rcvrAddr = this.props.data.address;
  	var amount = parseFloat(this.refs.amount.getDOMNode().value);

  	console.log(rcvrAddr, amount);
  	submitWFITrustTransaction(amount, rcvrAddr, null);
  },

  render: function() {
  	var modalUsername = this.props.data ? this.props.data.username : '';

  	return (
  		<div id="giftModal" className="modal">
        <div className="modal-content">
          <form onSubmit={this.submitTxn}>
          	<input
          		type="text"
          		id="user-input"
          		readOnly
          		value={modalUsername}
          	/>
          	<input
          		type="number"
          		placeholder="0"
          		ref="amount"
          	/>
          	<input
          		type="submit"
          	/>
          </form>
        </div>
      </div>
  	);
  }
});