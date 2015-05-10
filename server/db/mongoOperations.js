Meteor.methods({
  'addProduct': addProduct,
  'maxFlow': helpers.maxFlowBetweenAccounts
});

function addProduct(product) {}

var insertTransaction = function(sourceAddr, targetAddr, newLimit, neoOpResult, msg_json) {
	// TODO: REFACTOR; normalize db by storing txns in
		// Transaction table with references stored
		// in sourceUser and targetUser documents

	var sourceUser = Meteor.users.findOne({"profile.stellar.account_id": sourceAddr});
	var targetUser = Meteor.users.findOne({"profile.stellar.account_id": targetAddr});
	var sourceUsername = sourceUser.username;
	var targetUsername = targetUser.username;

	var currentLimit = neoOpResult[0].limit._data.data.prevAmount

	// parse memo from txn
	// TODO: refactor to handle more complicated memoData schemas
	var memoObj = Memo.parseMemo(msg_json);

	// update both user's arrays of related gifting/nixing txns
	Meteor.users.update({
		_id: sourceUser._id
	}, {
		$push: {
			transactions: {
				targetUsername: targetUsername,
				message: memoObj.memoData,
				limitChange: newLimit - currentLimit,
				ledger_index: msg_json.ledger_index,
				txnHash: msg_json.transaction.hash,
				txnDate: msg_json.transaction.date
			}
		}
	});

	Meteor.users.update({
		_id: targetUser._id
	}, {
		$push: {
			receivedTransactions: {
				sourceUsername: sourceUsername,
				message: memoObj.memoData,
				limitChange: newLimit - currentLimit,
				ledger_index: msg_json.ledger_index,
				txnHash: msg_json.transaction.hash,
				txnDate: msg_json.transaction.date
			}
		}
	});
};

mongoOperations = {
	insertTxn: insertTransaction
};