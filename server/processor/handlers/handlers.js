isValidTxn = function(msg) {
  return msg.hasOwnProperty('transaction') &&
    // msg_json.status === 'closed' &&   // msg_json.validated ???
    msg.meta.TransactionResult === 'tesSUCCESS';
};

messageHandler = {
  paymentHandler: function(msg_json) {
    console.log('paymentHandler:', msg_json);
  },

  trustHandler: function(msg_json) {
    var sourceAddr = msg_json.transaction.Account;
    var targetAddr = msg_json.transaction.LimitAmount.issuer;
    var newLimit = msg_json.transaction.LimitAmount.value;

    newLimit ? neoQueries.createEdge(sourceAddr, targetAddr, newLimit, dbCallback) : neoQueries.deleteEdge(sourceAddr, targetAddr, dbCallback);
  }
};

function dbCallback(err, res) {
  if (err) {
    console.log('error creating edge:', err);
  } else {
    console.log('successfully created edge:', res);
  }
}