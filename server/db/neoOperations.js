var Fiber = Npm.require('fibers');

Meteor.neo4j.methods({
  /*
  define queries that our clients can run

  each prop is a function that returns the string of our queries
  ex: return 'MATCH (a:Player {_id:{playerId}}) DELETE a'
    > this will match a doc in Player based on an obj with a prop
      playerId, then delete it
   */
});

var createUser = function(user, callback) {
  // saves:
  // user.id, username and stellar address
  var query = 'CREATE (:User {_id: {_id}, ' + 
    'username: {username}, ' + 
    'address: {address}})';

  //return neoQuerySync(query, null);
  neoQuery(query, {
    _id: user._id,
    username: user.username,
    address: user.profile.stellar.account_id
  }, function(err, res) {
    Fiber(function() {
      callback(err, res);
    }).run();
  });
};

var upsertEdge = function(sourceAddr, targetAddr, limit, callback) {
  var query = 'MATCH (s {address: {sourceAddr}}), ' + 
    '(t {address: {targetAddr}}) ' +
    'MERGE (s)-[limit:TRUST]->(t) ' +
    'ON MATCH SET limit.prevAmount = limit.amount, ' +
      'limit.amount = {amount} ' +
    'ON CREATE SET limit.amount = {amount}, ' + 
      'limit.prevAmount = 0, ' +
      'limit.source = {sourceAddr}, ' +
      'limit.target = {targetAddr} ' +
    'RETURN limit';

  neoQuery(query, {
    amount: limit,
    sourceAddr: sourceAddr,
    targetAddr: targetAddr
  }, function(err, res) {
    Fiber(function() {
      callback(err, res);
    }).run();
  });
};

neoOperations = {
  createUser: createUser,
  upsertEdge: upsertEdge
}