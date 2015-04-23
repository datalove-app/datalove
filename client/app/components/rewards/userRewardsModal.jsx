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

  	var userLimits = Meteor.neo4j.query('MATCH (s {address:{sourceAddr}})-[limits:TRUST]->(t {address:{targetAddr}}) RETURN limits', {
			sourceAddr: Session.get('myAddr'),
			targetAddr: rcvrAddr
		}).get() || [];

    var currentLimit = userLimits.length > 0 ? userLimits.limits[0].limit : 0;
  	var newLimit = Math.max(currentLimit + limitDelta, 0);

    var message = this.refs.message.getDOMNode().value;
    var memoObj = Memo.createMemo('', message);

  	submitWFITrustTransaction(newLimit, rcvrAddr, null, memoObj, null, function(err, res) {
      $(this.getDOMNode()).closeModal();
    }.bind(this));
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
              autofocus
          		type="number"
          		placeholder="0"
          		ref="amount"
          	/>
            <textarea
              rows="5"
              cols="50"
              ref="message"
            ></textarea>
          	<input
          		type="submit"
          	/>
          </form>
        </div>
      </div>
  	);
  }
});