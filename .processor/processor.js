var stellar = require('stellar-lib');
var Websocket = require('ws');
var ddp = require('./ddp_setup');     // our ddp funcs and cxn
// var login = require('ddp-login');  // https://github.com/vsivsi/ddp-login

var memoStore = require('./memostore.js');   // imports our unique txn classes

var base_fee = 200 * 10e6;

/*
optional:
	4. have user only get the txns pertaining to their Session.address
	5. have user fill out boxes and sign and submit txns and watch them come through
	
bonus:
	6. submit memodata
	7. store and read memodata to and from db
*/

////////////////////////////////////////////////////////////
/*	DDP INITIALIZATION STUFF	*/
////////////////////////////////////////////////////////////
// when publishing to meteor's site, update this info accordingly
// since this run locally but our meteor server won't

ddp.ddpClient.connect(function(err) {
	if (err) {
		console.log('There\'s been an error connecting to the Meteor server...');
		return;
	}
	console.log('Connecting to the Meteor server on port ' + ddp.ddpPort + '...');
});

var network_name = process.argv[2];
var ws;
if (network_name === 'local') {
	ws = new Websocket('ws://localhost:5006');  // untrusted access
} else if (network_name === 'live-stellar') {
	// SSL?
	ws = new Websocket('ws://live.stellar.org:9001');
} else if (network_name === 'test-stellar') {
  // SSL?
	ws = new Websocket('ws://test.stellar.org:9001')
} else {
		console.log('Incorrect usage')
}

ws.on('open', function() {
	console.log('Connecting to the ' + network_name + ' server using ws...');
	
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
	// console.log(msg_json);

	if (isValidTxn(msg_json)) {

    var memoObj = new memoStore.Memo(msg_json);
    if ( memoObj.memoType === 'wufi') {
      // console.log('its of memotype wufi');
      // console.log('memoobj looks like this: ' + JSON.stringify(memoObj));
      /*
      // if this is a basic STR txn... (this will go away later)

      if (msg_json.transaction.TransactionType === 'Payment') {
        var txn = new classes.BasicSTRTransaction(msg_json);
        insertTxn(txn);
      }
       */

      /////////////////////////////
      // if this is a UserInfo txn...
      // if (memoObj.memodata.type === 'user')
        // if Users.find({ _id: msg_json.transaction.Account })
          // FOLLOW UPDATE PROTOCOL":
        // else
          // FOLLOW CREATE PROTOCOL:

      // USERINFO implementation
      /*
      if (memoObj.memodata.type === 'user') {
        // console.log('its of memodata type user');

        var info;
        var userId = msg_json.transaction.Account;
        var txAmount = msg.transaction.Amount;

        if (txAmount === 5 * base_fee) {
          // 5 * base_fee signifies username registration == new public user
          info = new memoStore.UserInfo(msg_json, memoObj).createUserInfo();
          ddp.setUserInfo('insertUserInfo', info);

        } else if (txAmount === base_fee * .5) {
          info = new memoStore.UserInfo(msg_json, memoObj).updateUserInfo();

          // TODO: FINALIZE MEMOS so you can IF TEST the RIGHT METEOR METHODS
            // if test against the userInfo types (payment, profile, etc)

          if () {
            ddp.setUserInfo('setUserInfo', user, userId);
          }
        }
      }
      */

      if (memoObj.memoData.type === 'post') {

        var post = new memoStore.Post(msg, memoObj);
        ddp.insertPost(post);

      }
    }
	}

});

function isValidTxn(msg_json) {
  return msg_json.hasOwnProperty('transaction') &&
    // msg_json.status === 'closed' &&   // msg_json.validated ???
    msg_json.meta.TransactionResult === 'tesSUCCESS';
}


/////////////////////////////////////////////////////
/////////////////////////////////////////////////////

// closes the local stellard ledger every `timeout` ms
var timeout = 15000;
if (network_name === 'local') {
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

/////////////////////////////////////////////////////
/////////////////////////////////////////////////////