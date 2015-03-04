setStellarSession = function() {
  var user = Meteor.user();
  var addr = user.profile.stellar.account_id;
  var skey = user.profile.stellar.master_seed;
  Session.set('myAddr', addr);
  Session.set('mySecret', skey);
  remote.set_secret(addr, skey);
};

submitGenericTransaction = function(currencyCode, txnType) {
  // first three args are currencyCode and txnType
    // these are passed into bind
  // next are rcvrAddr, options (like amount), callback
  console.log(arguments);
  var amt = arguments[2],
    rcvrAddr = arguments[3],
    options = arguments[4],
    callback = arguments[5];


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

  if (txnType === 'payment') {
    txOptions.amount = amtNum;
  } else if (txnType === 'trustSet') {
    // TODO: FIX THIS, IT IS VERY WRONG
    txOptions.limitAmount = {};

  }

  tx[txnType](txOptions);

  console.log('bout to connect and submit', 'tx:', tx, 'remote:', remote);
  console.log('connected, bout to submit');
  tx.submit(function (err, res) {
    if (err) { throw err; }
    console.log('submitted');
    callback(res);
  });

};

submitSTRTransaction = submitGenericTransaction.bind(null, 'STR', 'payment');

submitWFITransaction = submitGenericTransaction.bind(null, 'WFI', 'payment');

submitWFITrustTransaction = submitGenericTransaction.bind(null, 'WFI', 'trustSet');
