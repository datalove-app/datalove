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
	// some unnecessary duplication going on here I believe...
	/*
	'click input#recipient-addr': function(e, template) {
		var self = document.getElementById('recipient-addr');
		self.value = '';
		self.classList.remove('empty')
	},

	'click input#send-amount': function(e, template) {
		var self = document.getElementById('send-amount');
		self.value = '';
		self.classList.remove('empty')
	},
	*/

	'click input#submit-txn': submitXRPTxn
});

Template.txnForms.events({

});