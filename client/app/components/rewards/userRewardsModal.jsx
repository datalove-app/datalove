UserRewardsModal = React.createClass({
	mixins: [ReactMeteor.mixin],

  getMeteorState: function() {
    return {};
  },

  submitTxn: function(event) {
  	/*
			get the change in limit amount
			get currentLimit || 0
			set amount = Math.max(currentLimit + limitDelta, 0);
			submit txn

  	 */

  	// TODO: REFACTOR to place neo4j query somewhere else more efficient
  		// depends on how reactive it is

  	event.preventDefault();

  	var rcvrAddr = this.props.data.address;
  	var limitDelta = parseFloat(this.refs.amount.getDOMNode().value);

  	var currentLimit = Meteor.neo4j.query('MATCH (s {address:{sourceAddr}})-[limits:TRUST]->(t {address:{targetAddr}}) RETURN limits', {
			sourceAddr: Session.get('myAddr'),
			targetAddr: rcvrAddr
		}).get().limits[0].limit;

  	var newLimit = Math.max(currentLimit + limitDelta, 0);
  	submitWFITrustTransaction(newLimit, rcvrAddr, function(err, res) {
  		// do something here
  	});
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