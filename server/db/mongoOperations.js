Meteor.methods({
  'addProduct': addProduct,

  'maxFlow': helpers.maxFlowBetweenAccounts
});

function addProduct(product) {
  Products.insert(product);
}

var insertTransaction = function(sourceAddr, targetAddr, newLimit, msg_json) {

	var currentLimit = Meteor.neo4j.query('MATCH (s {address:{sourceAddr}})-[limits:TRUST]->(t {address:{targetAddr}}) RETURN limits', {
			sourceAddr: sourceAddr,
			targetAddr: targetAddr
		}).get().limits[0].limit;

	var targetUsername = Meteor.users.findOne({"profile.stellar.account_id": targetAddr}).username;

	// parse memo from txn
	// TODO: refactor to more complicated memoData schemas
	var memoObj = Memo.parseMemo(msg_json);

	Meteor.users.update({
		"profile.stellar.account_id": sourceAddr
	}, {
		$push: {
			transactions: {
				targetUsername: targetUsername,
				message: memoObj.memoData,
				limitChange: newLimit - currentLimit,
				ledger_index: msg_json.ledger_index,
				ledger_hash: msg_json.ledger_hash,
				txnHash: msg_json.transaction.hash,
				txnDate: msg_json.transaction.date
			}
		}
	});
};

mongoOperations = {
	insertTxn: insertTransaction
};