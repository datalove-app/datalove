var request = Meteor.npmRequire('request');

Accounts.onCreateUser(demoRegistrationHook);

/*
 This function will:
 - create the stellar key and address
 - save them alongside their username and password in the db
 - create a vertex in neo4j for the user

 For demo purposes:
 - we are allowing the key to stored in plaintext on the server
 - we are not putting it in a blob encrypted by a user password

 In the future, this function (or others like it) will also
 register the user on the Stellar network through a special
 memo in a transaction sent immediately on signup.
 */

var postSync = Async.wrap(request.post);
var getSync = Async.wrap(request.get);
var getTestStellar = function(stellarAccount) {
  return getFreeStellar ? getSync({url: 'https://api-stg.stellar.org/friendbot?address=' + stellarAccount.account_id}) : null;
};

function demoRegistrationHook(options, user) {
  console.log('running registration hook');

  // have stellard create us a wallet full of stellar keys and seeds
  var res = postSync({url: 'https://test.stellar.org:9002', form: JSON.stringify({"method": "create_keys"})});

  var stellarAccount = JSON.parse(res.body).result;
  delete stellarAccount.status;

  // get free testnet stellar from SDF
  var getRes = getTestStellar(stellarAccount);

  user.profile = options.profile || {};
  // TODO: encrypt stellar data before storage
  user.profile.stellar = stellarAccount;
  user.transactions = [];
  user.receivedTransactions = [];

  neoOperations.createUser(user, function(){});
  return user;
}
