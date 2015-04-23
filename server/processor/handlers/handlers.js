var Fiber = Npm.require('fibers');

isValidTxn = function(msg) {
  return msg.hasOwnProperty('transaction') &&
    // msg_json.status === 'closed' &&   // msg_json.validated ???
    msg.meta.TransactionResult === 'tesSUCCESS';
};

messageHandler = {
  paymentHandler: function(msg_json) {
    console.log('[PROCESSOR] received [payment] message from txn network');
  },

  trustHandler: function(msg_json) {
    var sourceAddr = msg_json.transaction.Account;
    var targetAddr = msg_json.transaction.LimitAmount.issuer;
    var newLimit = parseFloat(msg_json.transaction.LimitAmount.value);

    if (msg_json.transaction.LimitAmount.currency !== 'WFI') { 
      return; 
    }

    console.log('[PROCESSOR] received [gifting] message from txn network');

    neoOperations.editEdge(sourceAddr, targetAddr, 
      newLimit, function(err, res) {
      // after creating/editing edge, insert txn into User obj
      mongoOperations.insertTxn(sourceAddr, targetAddr, newLimit, msg_json);
    });
  }
};

function dbCallback(err, res) {
  if (err) {
    console.log('error creating edge:', err);
  } else {
    console.log('successfully created edge:', res);
  }
}