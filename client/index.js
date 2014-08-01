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

Template.sendSTR.events({
	'click input#submit-txn': submitSTRTxn
});

Template.config.helpers({
	myAddr: function() {
		return Session.get('myAddr')
	}
})

Template.config.events({
	'click input#submit-config' : updateConfig
})

Template.txnForms.events({

});