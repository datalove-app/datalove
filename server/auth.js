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
var createQueryString = function(user) {
  return 'CREATE (:User {_id:"' + user._id +
      '",' + 'username:"' + user.username +
      '",' + 'address:"' + user.profile.stellar.account_id +
      '",' + 'secret:"' + user.profile.stellar.master_seed + '"})';
};

function demoRegistrationHook(options, user) {
  console.log('running registration hook');

  var res = postSync({url: 'https://test.stellar.org:9002', form: JSON.stringify({"method": "create_keys"})});

  var stellarAccount = JSON.parse(res.body).result;
  delete stellarAccount.status;

  user.profile = options.profile || {};
  user.profile.stellar = stellarAccount;

  // should save address and mongoId
  var queryString = createQueryString(user);
  Meteor.N4JDB.query(queryString, null, function(err, res) {
    if (err) {
      console.log('error in creating user in neo4j');
    } else {
      console.log('successfully created user in neo4j:', res);
    }
  });

  return user;
}
