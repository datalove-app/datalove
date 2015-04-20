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
  // user.id and username
  // stellar address and secret key
  var query = 'CREATE (:User {_id:"' + user._id +
    '",' + 'username:"' + user.username +
    '",' + 'address:"' + user.profile.stellar.account_id + '"})';

  //return neoQuerySync(query, null);
  neoQuery(query, null, callback);
};

var createEdge = function(sourceAddr, targetAddr, limit, callback) {
  // handle simple trustSet (that is, create/overwrite)
    // TODO: allow users to add or remove trustlines

  // use MERGE to update/create trustline:
    // http://neo4j.com/docs/stable/query-merge.html
  var query = 'MATCH (s {address:"' + sourceAddr +
    '"}),(t {address:"' + targetAddr + '"})' +
    'CREATE (s)-[:TRUST {limit:' + limit + '}]->(t)';

  neoQuery(query, null, callback);
};

var deleteEdge = function(sourceAddr, targetAddr, callback) {
  var query = 'MATCH (s {address:"' + sourceAddr +
    '"})-[limit]->(t {address:"' + targetAddr + '"})' +
    'DELETE limit';

  neoQuery(query, null, callback)
};

neoOperations = {
  createUser: createUser,
  createEdge: createEdge,
  deleteEdge: deleteEdge
}