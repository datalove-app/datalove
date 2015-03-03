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

function demoRegistrationHook(options, user) {
  console.log('running registration hook');
  var stellar = 'stellar props go here';

  user.profile = options.profile || {};
  user.profile.stellar = stellar;

  Meteor.N4JDB.query('CREATE (:User {_id:"' + user._id + '"})', null, function(err, res) {
    if (err) {
      console.log('error in creating user in neo4j');
    } else {
      console.log('successfully created user in neo4j:', res);
    }
  });

  return user;
}
