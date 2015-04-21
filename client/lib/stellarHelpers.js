setStellarSession = function() {
  var user = Meteor.user();
  var addr = user.profile.stellar.account_id;
  var skey = user.profile.stellar.master_seed;
  Session.set('myAddr', addr);
  Session.set('mySecret', skey);
  remote.set_secret(addr, skey);
};

var submitGenericTransaction = function(currencyCode, txnType, amt, rcvrAddr, options, callback) {
  if (typeof amt !== 'number' || currencyCode.length > 3) {
    return;
  }

  options = options || null;
  callback = callback || function() { console.log('running default callback, err/res are:', arguments); };

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

  tx.submit(function (err, res) {
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