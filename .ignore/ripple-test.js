/*

    rootAddr = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"; 
    rootSec = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";
    
    function sign_share_tx(secret, senderAddr, destAddr, amount) {
        var tx_JSON, tx, unsigned;
        tx_JSON = {
            "TransactionType": "TrustSet",
            "Account": senderAddr,
            "LimitAmount": {
                "currency": "WFI",
                "value": amount,
                "issuer": destAddr
            }
        };
        tx = new ripple.Transaction();
        tx.tx_json = tx_JSON;
        tx._secret = secret;
        tx.complete();
        unsigned = tx.serialize().to_hex();
        tx.sign();
        return tx.serialize().to_hex();
    }
    tx_blob = sign_share_tx(rootSec, rootAddr, "r3kmLJN5D28dHuH8vZNUZpMC43pEHpaocV", 100);
    console.log(tx_blob);
})();
*/

/*
function signtx(secret, tx_in) {
  var tx_JSON = JSON.parse(tx_in);
  var tx = new ripple.Transaction();
  tx.tx_json = tx_JSON;
  tx._secret = secret;
  tx.complete();
  var unsigned = tx.serialize().to_hex();
  tx.sign();
  return (tx.serialize().to_hex());
}
  var tx = '{ "TransactionType" : "Payment", 
              "Account" : "raSv7ZM4KvK99REGHfPSGn8QpdpJWtNTrN", 
              "Destination" : "r3kmLJN5D28dHuH8vZNUZpMC43pEHpaocV", 
              "Amount" : "10", "Fee" : "10", "Sequence" : 1 }';
  var signature = signtx('sxxxxxxxxxxxxxx',tx);

*/


// var ripple = Meteor.require('ripple-lib');

var rootAddr = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"; 
var rootSecret = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";

var Amount, Remote, remote;

Amount = ripple.Amount;

Remote = ripple.Remote;
remote = new Remote({
	trusted: false,
	local_signing: true,
	local_fee: true,
	fee_cushion: 15,
	max_fee: 100,
	servers: [ {
		host: "127.0.0.1",
		// port: 5006,
		port: 6006,
		secure: false
	} ]
});

///////////////////////////////////////////////////////
///////////////////////////////////////////////////////
// txn stuff

/*
var rcvrAddr  = 'rPJP6M7BmmxVFRUsRvu1x6vSar46tsnXVH';
var rcvrSecret = 'ssHQXj2YY5fm5VqbBNdp9o9rKw7RB';
var test_amount     = Amount.from_human('20000XRP');

remote.set_secret(rootAddr, rootSecret);

var transaction = remote.transaction();

transaction.payment({
	from: rootAddr, 
	to: rcvr, 
	amount: test_amount
});

remote.conect(function(err) {
	if (err) {
		console.log('there\'s been an error');
	} else {
		console.log('Connected to the localhost server...');
	}
});


transaction.submit(function(err, res) {

	if (err) {
		alert('error: ' + err);
	} else if (res) {
		alert('success: ' + res);
	}

});

*/



/*
# testing the share function
tx_blob = sign_share_tx(rootSec, rootAddr, "r3kmLJN5D28dHuH8vZNUZpMC43pEHpaocV", 100)
console.log(tx_blob)
# share_submit_request = remote.request_submit(tx_blob)
*/












