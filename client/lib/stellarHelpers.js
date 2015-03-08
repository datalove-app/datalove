setStellarSession = function() {
  var user = Meteor.user();
  var addr = user.profile.stellar.account_id;
  var skey = user.profile.stellar.master_seed;
  Session.set('myAddr', addr);
  Session.set('mySecret', skey);
  remote.set_secret(addr, skey);
};

submitGenericTransaction = function(currencyCode, txnType, amt, rcvrAddr, options, callback) {
  // first three args are currencyCode and txnType
    // these are passed into bind
  // next are rcvrAddr, options (like amount), callback
  if (typeof amt !== 'number' ||
    currencyCode.length > 3) {
    return;
  }

  callback = callback || function() { console.log('args:', arguments); };

  var amtNum = Amount.from_human(amt + currencyCode);
  var tx = remote.transaction();

  var txOptions = Object.create(options);
  txOptions.from = Session.get('myAddr');
  txOptions.to = rcvrAddr;

  tx[txnType](txOptions);

  if (txnType === 'payment') {
    tx.tx_json.amount = amtNum;
  } else if (txnType === 'trustSet') {
    tx.tx_json.LimitAmount = {
      currency: currencyCode,
      value: amt.toString(),
      issuer: rcvrAddr
    };
  }

  console.log('bout to connect and submit', 'tx:', tx, 'remote:', remote);
  tx.submit(function (err, res) {
    //if (err) { throw err; }
    console.log('submitted');
    callback(err, res);
  });

};

retrieveAccountInfo = function(rcvrAddr, callback) {
  remote.request_account_info(rcvrAddr, callback);
};

submitSTRTransaction = submitGenericTransaction.bind(null, 'STR', 'payment');

submitWFITransaction = submitGenericTransaction.bind(null, 'WFI', 'payment');

submitWFITrustTransaction = submitGenericTransaction.bind(null, 'WFI', 'trustSet');
