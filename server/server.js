Meteor.methods({
	
	'addTxn': function(txn) {
		console.log('Adding a txn to our collection...');	
		Transactions.insert(txn);
	}
	
});