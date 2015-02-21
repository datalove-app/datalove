var DDPClient = require('ddp');
// var login = require('ddp-login');  // https://github.com/vsivsi/ddp-login

var ddpHost = 'localhost';
var ddpPort = 8080;

////////////////////////////////////////////////////////////
/*	DDP INITIALIZATION STUFF	*/
////////////////////////////////////////////////////////////
// when publishing to meteor's site, update this info accordingly
// since this run locally but our meteor server won't

var ddpClient = new DDPClient({
  // production notes:
  /*

   // logging in with username (NOTE: use ddp-login)
   ddpclient.call(
   "login",
   [{
   user : { username : "username" },
   password : "password"
   }],
   function (err, result) { ... }
   );

   //

   */
  host: ddpHost,
  port: ddpPort,
  auto_reconnect: true,
  auto_reconnect_timer: 500,
  use_ssl: false,
  maintain_collections: false
});

// define the ddp call to the server
/*
function insertTxn(txn) {
  ddpClient.call(
    'addTxn',     // method name
    [txn],        // array of parameters

    // returns the method call results
    function(err, result) {
      if (err) {
        console.log('There was an error calling addTxn: ' + err);
      } else {
        // result returns undefined
        console.log('Successfully called and returned from addTxn: ' + result);
      }
    },

    // fires when the server is finished with the called method
    function() {
      console.log('finished calling insertTxn');
    }
  );
}
*/

function insertPost(post) {
  ddpClient.call(
    'addPost',
    [post],
    function(err, res) {
      if (err) {
        console.log('There was an error calling addPost: ', err);
      } else {
        console.log('Successfully called and returned from addPost: ', res);
      }
    },
    function() {
      console.log('finished calling insertPost');
    }
  );
}

/*
function setUserInfo(meteorMethod, userInfo, id) {
  ddpClient.call(
    meteorMethod,
    [id, userInfo],
    function(err, res) {
      if (err) {
        console.log('There was an error creating/updating the user: ' + err.toString());
      } else {
        console.log('Successfully called and returned from upsertUser: ' + res);
      }
    },
    function() {
      console.log('finished calling upsertUser')
    }
  )
}
*/

// EXPORTS

exports.ddpPort = ddpPort;
exports.ddpClient = ddpClient;

exports.insertPost = insertPost;
// exports.setUserInfo = setUserInfo;