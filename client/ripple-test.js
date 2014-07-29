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

Meteor.startup(function() {
	rootAddr = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"; 
	rootSecret = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";

	var Amount = ripple.Amount;
	var Remote = ripple.Remote;

	remote = new Remote({
		trusted: false,
		local_signing: true,
		local_fee: true,
		fee_cushion: 15,
		max_fee: 150,
		servers: [ {
			host: "127.0.0.1",
			port: 5006,
			secure: false
		} ]
	});

	remote.connect(function(err) {
		console.log('Connecting to the local rippled...');

		rootAddr = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"; 
		rootSecret = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";
		rcvrAddr  = 'rPJP6M7BmmxVFRUsRvu1x6vSar46tsnXVH';
		rcvrSecret = 'ssHQXj2YY5fm5VqbBNdp9o9rKw7RB';
		var test_amount = Amount.from_human('200XRP');

		remote.set_secret(rootAddr, rootSecret);

		tx = remote.transaction();

		// one of a few types of tx
		tx.payment({
			from: rootAddr, 
			to: rcvrAddr, 
			amount: test_amount
		});

		tx.submit(function(err, res) {
			if (err) {
			console.log('error: ' + err.result_message);
			} else {
			console.log('success!');
			}
		});

	});

	/* 	SOME FUNCTIONS TO REMEMBER

	request_server_info(callback)
		can tell you the reserve costs
		they should be in the response obj

	request_account_info(addr, callback)
		tells you an xrp balance

	since you can't return anything out of a callback, you'll have to call a second function within the callback to do the stuff to the value you would have otherwise returned





	*/

	/*
	# testing the share function
	tx_blob = sign_share_tx(rootSec, rootAddr, "r3kmLJN5D28dHuH8vZNUZpMC43pEHpaocV", 100)
	console.log(tx_blob)
	# share_submit_request = remote.request_submit(tx_blob)
	*/
});












