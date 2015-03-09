Meteor.neo4j.methods({
  /*
  define queries that our clients can run

  each prop is a function that returns the string of our queries
  ex: return 'MATCH (a:Player {_id:{playerId}}) DELETE a'
    > this will match a doc in Player based on an obj with a prop
      playerId, then delete it

   */

});

var neoQuery = Meteor.N4JDB.query;
var neoQuerySync = Async.wrap(Meteor.N4JDB.query);

neoQueries = {
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

maxFlow = function(sourceAddr, targetAddr, flowLimit, callback) {
  flowLimit = flowLimit || 2;
  var query = 'MATCH (:User {address:"' + sourceAddr +
    '"})-[limit:TRUST*..' + flowLimit + ']->' +
    '(:User {address:"' + targetAddr +
    '"}) RETURN limit';

  var res = neoQuerySync(query, null);
  console.log(res);
};

var flowGraph = {};

Meteor.methods({

  maxFlowTest: function (sourceAddr, targetAddr, flowLimit, callback) {
    flowLimit = flowLimit || 2;
    var query = 'MATCH (:User {key:"' + sourceAddr +
      '"})-[limit:TRUST*..3]->' +
      '(:User {key:"' + targetAddr +
      '"}) RETURN limit';

    var paths = neoQuerySync(query, null);
    var maxFlow = 0;

    // for each path
    paths.forEach(function(obj) {

      var path = obj.limit;
      var pathCapacity = Number.POSITIVE_INFINITY;

      path.forEach(function(edge) {
        var edgeId = edge._data.metadata.id;
        var limit = edge._data.data.limit;

        // add edges to flowGraph
        addToFlowGraph(edgeId, limit);
      });

      // find lowest limit (capacity)
      path.forEach(function(edge) {
        var edgeId = edge._data.metadata.id;
        console.log(flowGraph[edgeId].capacity, pathCapacity);
        if (flowGraph[edgeId].capacity < pathCapacity) {
          pathCapacity = flowGraph[edgeId].capacity;
        }
      });

      // subtract capacity from each edge's capacity
      path.forEach(function(edge) {
        var edgeId = edge._data.metadata.id;
        var limit = flowGraph[edgeId].limit;
        var capacity = flowGraph[edgeId].capacity;

        flowGraph[edgeId].capacity -= pathCapacity;
      });

      // add capacity to flow
      console.log(pathCapacity);
      maxFlow += pathCapacity;
    });

    resetFlowGraph();
    return [paths, flowGraph, maxFlow];
  }

});

function addToFlowGraph(edgeId, limit) {
  // called on every edge
  // if edge exist
    // return
  // else
    // set edge to {limit:limit, capacity: limit}

  flowGraph[edgeId] = flowGraph[edgeId] || {
    limit: limit,
    capacity: limit
  }
}

function resetFlowGraph() {
}
