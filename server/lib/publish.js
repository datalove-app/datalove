Meteor.publish('userData', function() {
	if (!this.userId) { return null; }
	return Meteor.users.find(this.userId, {
		transactions: 1,
		receivedTransactions: 1
	});
});
