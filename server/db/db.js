Meteor.neo4j.methods({
  /*
  define queries that our clients can run

  each prop is a function that returns the string of our queries
  ex: return 'MATCH (a:Player {_id:{playerId}}) DELETE a'
    > this will match a doc in Player based on an obj with a prop
      playerId, then delete it

   */

});

neoOperations = {
  createUser: function(user, callback) {
    // saves:
    // user.id and username
    // stellar address and secret key
    var query = 'CREATE (:User {_id:"' + user._id +
      '",' + 'username:"' + user.username +
      '",' + 'address:"' + user.profile.stellar.account_id +
      '",' + 'secret:"' + user.profile.stellar.master_seed + '"})';

    //return neoQuerySync(query, null);
    neoQuery(query, null, callback);
  },

  createEdge: function(sourceAddr, targetAddr, limit, callback) {
    // handle simple trustSet (that is, create/overwrite)
      // TODO: allow users to add or remove trustlines
    var query = 'MATCH (s {address:"' + sourceAddr +
      '"}),(t {address:"' + targetAddr + '"})' +
      'CREATE (s)-[:TRUST {limit:' + limit + '}]->(t)';

    neoQuery(query, null, callback);
  },

  deleteEdge: function(sourceAddr, targetAddr, callback) {
    var query = 'MATCH (s {address:"' + sourceAddr +
      '"})-[limit]->(t {address:"' + targetAddr + '"})' +
      'DELETE limit';

    neoQuery(query, null, callback)
  }
};

