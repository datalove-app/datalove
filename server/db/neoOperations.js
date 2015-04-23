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

// TODO: REFACTOR to use opts

var createUser = function(user, callback) {
  // saves:
  // user.id, username and stellar address
  var query = 'CREATE (:User {_id:"' + user._id +
    '",' + 'username:"' + user.username +
    '",' + 'address:"' + user.profile.stellar.account_id + 
    '"})';

  //return neoQuerySync(query, null);
  neoQuery(query, null, callback);
};

var createEdge = function(sourceAddr, targetAddr, limit, callback) {

  var query = 'MATCH (s {address:"' + sourceAddr +
    '"}),(t {address:"' + targetAddr + '"})' +
    'MERGE (s)-[limit:TRUST]->(t)' +
    ' ON MATCH SET limit.limit = ' + limit +
    ' ON CREATE SET limit.limit = ' + limit + 
      ', limit.source = "' + sourceAddr +
      '", limit.target = "' + targetAddr + '";';

  neoQuery(query, null, callback);
};

var deleteEdge = function(sourceAddr, targetAddr, callback) {
  var query = 'MATCH (s {address:"' + sourceAddr +
    '"})-[limit]->(t {address:"' + targetAddr + '"})' +
    'DELETE limit';

  neoQuery(query, null, callback);
};

var editEdge = function(sourceAddr, targetAddr, limit, callback) {
  var query;
  if (limit === 0) {
    query = 'MATCH (s {address:"' + sourceAddr +
    '"})-[limit]->(t {address:"' + targetAddr + '"})' +
    'DELETE limit'
  } else {
    query = 'MATCH (s {address:"' + sourceAddr +
    '"}),(t {address:"' + targetAddr + '"})' +
    'MERGE (s)-[limit:TRUST]->(t)' +
    ' ON MATCH SET limit.limit = ' + limit +
    ' ON CREATE SET limit.limit = ' + limit + 
      ', limit.source = "' + sourceAddr +
      '", limit.target = "' + targetAddr + '";';
  }

  neoQuery(query, null, function(err, res) {
    Fiber(function() {
      callback(err, res);
    }).run();
  });
}

neoOperations = {
  createUser: createUser,
  createEdge: createEdge,
  deleteEdge: deleteEdge,
  editEdge: editEdge
}