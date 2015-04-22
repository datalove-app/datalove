var WebSocket = Meteor.npmRequire('ws');

/*
optional:
	4. have user only get the txns pertaining to their Session.address
	5. have user fill out boxes and sign and submit txns and watch them come through
	
bonus:
	6. submit memodata
	7. store and read memodata to and from db
*/

ws = new WebSocket(stellardCxn.url);

ws.on('open', function() {
	console.log('[PROCESSOR] Connecting to the ' + stellardCxn.name + ' stellard using ws...');
	
	// subscribes us to ledger close events
	// ws.send('{"command": "subscribe", "id": 0, "streams": ["ledger"]}');
	
	// subscribes to txn stream, however you only get the txns on ledger close
	ws.send('{"command": "subscribe", "streams": ["transactions"]}');
});

////////////////////////////////////////////////////////////
/* THE GOOD STUFF
this is where we'll receive the msgs from the ledger
  and parse them for storage or ignoring
*/
////////////////////////////////////////////////////////////

ws.on('message', function(msg) {

	var msg_json = JSON.parse(msg);
  if (!isValidTxn(msg_json)) {
    return;
  }

  // BEGIN message handlers
  //if (memoObj.memoData.type === 'post') {
  //  console.log('msg looks like:', memoObj);
  //  //var post = new memoStore.Post(msg, memoObj);
  //}

  if (msg_json.transaction.TransactionType === 'Payment') {
    // handle only payment of WFI
    messageHandler.paymentHandler(msg_json);
    return;
  } else if (msg_json.transaction.TransactionType === 'TrustSet') {
    // handles only trustSets denominated in WFI
    messageHandler.trustHandler(msg_json);
    return;
  }

});

/////////////////////////////////////////////////////
/////////////////////////////////////////////////////

// closes the local stellard ledger every `timeout` ms
var timeout = 15000;
if (stellardCxn.name === 'local') {
	(function (interval) {
		var Remote = stellar.Remote;

		var remote = new Remote({
			servers: [
				{
					host: "127.0.0.1",
					port: 6006,	// admin access
					secure: false
				}
			]
		});

		remote.connect(
			// close the ledger every 10 seconds:
			setInterval(function () {
				remote.ledger_accept();
				console.log('\n## Closing ledger... ##');
			}, interval)
		);
	})(timeout);
}
