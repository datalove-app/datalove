/*
this should hold our classes for DB ENTRIES FROM MEMOS
from our responses from the ledger, we will create appropriate objs containing:
	txn information
	anything necessary to displaying the post
		(info like the image/video name may need to be provided separately)
		this would mean that video and image sharing will come after textpost and (maybe) article sharing

 */

var stellar = require('stellar-lib');
var utils = stellar.utils;

exports.BasicSTRTransaction = BasicSTRTransaction;
exports.UserInfo = UserInfo;
exports.Memo = Memo;
exports.Post = Post;

function Memo(msg) {
  // gets passed in the entire msg to get the Memo obj of the Memos array
  // later, when bundled into a lib, this will be a func w/in the Memo class

  var memo = msg.transaction.Memos[0].Memo;
  // as of wufi-schema v0.1, memoType will always be 'wufi'
  this.memoType = JSON.parse(utils.hexToString(memo.MemoType));
  this.memoData = JSON.parse(utils.hexToString(memo.MemoData));
}

function BasicSTRTransaction(msg) {
	// this is a basic class for storing simple STRTransactions from ledger
	// the main changes you'll see will be additions to the Memo obj of the Memos array
  // msg == json of the ledger txn msg

  var day_zero = 946684800;

	this.type = 'BasicSTRTransaction';    // ??? why ???

	this._id = msg.transaction.hash;
	this.sender = msg.transaction.Account;
	this.receiver = msg.transaction.Destination;
	this.amount = msg.transaction.Amount;
	this.ledger = msg.ledger_index;
	this.date = new Date((day_zero + msg.transaction.date) * 1000);

	var memoObj = new Memo(msg);
	this.memotype = memoObj.memotype;
	this.memodata = memoObj.memodata;
}

function UserInfo(msg, memoObj) {
  /*
  this class does two things:
    validates the memo and txnAmount as a genuine Wufi-UserInfo txn
    creates our User obj, sent to the Meteor's Users collection

  ** WHEN USED: **
  * make sure you validate the msg BEFORE passing it in

  two halves of data:
    what comes from the memos that needs to be stored
    what comes from the rest of the txn which proves the authenticity of the entry's info (txnAmount, sourceAddress, etc)

  both halves are needed when:
    creating an obj for a new user
    creating an obj that will be used to update a user
  */

  // var day_zero = 946684800;

  //////////////////////
  // memo info:
  // derived from memoObj.memodata;

  // TODO: **** DEFINE OUR METHOD OF STORING IDENTITY INFO IN TXNS, IT IS THE NECESSARY NEXT STEP ****

  /////////////////////
  // authenticity info:
  // this.hash = msg.transaction.hash;

  // these could go into UserInfo.prototype, but it wouldn't change how their used and called
  this.createUserInfo = function() {
    // creates the obj that will be the user's first db entry
    return this.info = {
      // initialize the first user db entry with expected fields:
        // payments, basics, profiles, photos, misc, etc

      _id: msg.transaction.Account,
      username: memoObj.memodata.username,
      basics: memoObj.memodata.basics || {},    // should be obj w/ bio, location, first and last names

      payments: [{
        type: 'stellar',
        address: msg.transaction.Account,
        proof: {    // proves that this username is associated w/ this addr
          ledger_index: msg.ledger_index,
          tx_hash: msg.transaction.hash
        }
      }],

      profiles: [],

      openspecs_version: '0.3'
      // wufi_schema_version: '0.1'
    }
  };

  this.updateUserInfo = function() {

    var memodata = memoObj.memodata;

    // if memodata.hasProp(), which can be:
      // basics     // bio, names, location
      // payment    // btc addr, venmo username, etc + proofs
      // profile    // fb, tw, sc, ig, + proofs
      // >> then, call the right func for the right update

    if (memodata.hasOwnProperty('basics')) {

    }

  }


}

function Post(msg, memoObj) {
  /*
  this class does two things:
    validates the memo and txnAmount as a genuine Wufi-Post txn
    creates our User obj, sent to the Meteor's Posts collection

  should be a base class for comments, textless
  */
  /*
  validation:
    each post must get from txn:
      sender
      rcvr
      new total TrustLine amount for the rcvr
      data:
        parentID      // so we know where this post should be shown
        text          // the text of the msg, probably 200 char limit

    each post must also have fields for/store this data for presentation and analysis:
      children        // at least an arr of IDs that reply to this post

  */

  var day_zero = 946684800;

  // this.type = 'BasicSTRTransaction';    // ??? why ???
  this._id = msg.transaction.hash;
  // could be shortened so that the parentID in ledger Memo is shorter
  this.senderID = msg.transaction.Account;
  this.receiverID = msg.transaction.Destination;

  this.amount = msg.transaction.Amount;
  this.ledgerIndex = msg.ledger_index;
  this.date = new Date((day_zero + msg.transaction.date) * 1000);

  // this is where we store the memo properties
  // TODO: this is what will change when we come closer to finalizing the post-memo-schema and creating subclasses for deals, comments, etc from this
  memoObj = memoObj || new Memo(msg);
  this.parentID = memoObj.memoData.parentID || null;
  this.text = memoObj.memoData.text || null;
  // in ddp_setup or in Meteor.methods, we'll use the parentID to add this post to the correct position of the it's post-thread (post + comment tree)
  // this.children = {};
}

/////////////////////////////////
// these are validation funcs
// these run in processor.js
// and verify that the memos being pulled from the ledger are wufi-compliant
// and ready to be included and shared with users
/////////////////////////////////

var IdentityMemo = function() {
  /* called by client when:
   // they first register their username (
   // they update profile with bio, name, location

   it needs to create the data Object to be stringified and hexified
   then, store it as one of it's own properties

   */

  /*
   needs to keep track of characters in memo
   needs to store in ledger:
   // 1. username
   // 2. profile:
   // > username
   // > proof url
   //
   */

  // TODO: **** DEFINE THE UPDATE/CREATE MEMO ****

  this.setUsername = function() {
    // at the end of this func: we must
    // create this.Memo as a plaintext Memo obj

  };

  /*
   after everything is set, run:
   this.memo = new Memo(this.data)
   this.getMemo = memo.getMemo;

   then when you're ready to share wufi:

   */

};

var ShareMemo = function(data) {

  // do some validation on data to make sure it is ShareMemo compliant

  Memo.call(this, data);
};


////////////////////////////////////////////
////////////////////////////////////////////

// EXAMPLE OF SUCCESSFUL TXN RESPONSE (to subscribing client)
/*
{ engine_result: 'tesSUCCESS',
  engine_result_code: 0,
  engine_result_message: 'The transaction was applied.',
  ledger_hash: '96025F2881FFA900F475D5B316EDC199CEEAABBF19F5D59CFB41726CF4CA0B9D',
  ledger_index: 3,
  meta:
    { AffectedNodes: [ [Object], [Object] ],
      TransactionIndex: 0,
      TransactionResult: 'tesSUCCESS'
    },
  status: 'closed',
	transaction:
    { Account: 'ganVp9o5emfzpwrG5QVUXqMv8AgLcdvySb',
      // we input 10000 into submitTxn
      Amount: '10000000000',
      Destination: 'gM4Fpv2QuHY4knJsQyYGKEHFGw3eMBwc1U',
      Fee: '15',
      Flags: 2147483648,
      LastLedgerSequence: 11,

      Memos: [ {
				Memo: {
					MemoType: 'str',
					MemoData: 'str'
				}
      } ],

      Sequence: 1,
      SigningPubKey: 'BE3900393891A2A2244E28A82C43BA94CA94DD6BFE36D523576A22BFF86055D4',
      TransactionType: 'Payment',
      TxnSignature: 'F26A24E0763800034FD08342E4D029DC8C258377898B66542A57FF24DF9A3DCB9CD03300DA3B0918FE4216543450152AC7299577FBF9209E09B364ED75EBD109',
      date: 462938250,
      // hash is 32-digit hex
      hash: '1B5BA850F4A98A452BDDE6A2A2D607BB990D4921F66341D8F5F01E16765A9894'
    },
  type: 'transaction',
  validated: true
}

 */
// EXAMPLE OF SUCCESSFUL TXN RESPONSE (on stellard server)
/*
{ id: 3,
	result:
	{ engine_result: 'tesSUCCESS',
		engine_result_code: 0,
		engine_result_message: 'The transaction was applied.',
		tx_blob: '12000022800000002400000001201B0000000B6140000002540BE40068400000000000000F7320BE3900393891A2A2244E28A82C43BA94CA94DD6BFE36D523576A22BFF86055D47440F26A24E0763800034FD08342E4D029DC8C258377898B66542A57FF24DF9A3DCB9CD03300DA3B0918FE4216543450152AC7299577FBF9209E09B364ED75EBD109811437B1B26BE3C91C55D51586C3F0E5C4B03E9CEA7F8314DF8286CDBB009AA5C29F526B5C3B4C480B44EABEF9EA7C04747970657D0474657874E1F1',
		tx_json:
		{ Account: 'ganVp9o5emfzpwrG5QVUXqMv8AgLcdvySb',
			Amount: '10000000000',
			Destination: 'gM4Fpv2QuHY4knJsQyYGKEHFGw3eMBwc1U',
			Fee: '15',
			Flags: 2147483648,
			LastLedgerSequence: 11,
			Memos: [Object],
			Sequence: 1,
			SigningPubKey: 'BE3900393891A2A2244E28A82C43BA94CA94DD6BFE36D523576A22BFF86055D4',
			TransactionType: 'Payment',
			TxnSignature: 'F26A24E0763800034FD08342E4D029DC8C258377898B66542A57FF24DF9A3DCB9CD03300DA3B0918FE4216543450152AC7299577FBF9209E09B364ED75EBD109',
			hash: '1B5BA850F4A98A452BDDE6A2A2D607BB990D4921F66341D8F5F01E16765A9894' }
	},
	status: 'success',
	type: 'response'
}
*/
