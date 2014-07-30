Template.txns.helpers({
	txnsList: function() {
		return Transactions.find({}, {sort: {date: -1}});
	}
});

Template.txn.helpers({
	date: convertDate,

	amount: function() {
		return this.amount/1e6
	}
});

Template.sendXRP.events({
	'click input#recipient-addr': function(e, t) {
		document.getElementById('recipient-addr').value = ''
	},

	'click input#send-amount': function(e, t) {
		document.getElementById('send-amount').value = ''
	},

	'click input#submit-txn': submitXRPTxn
});
