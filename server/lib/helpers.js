// TODO: rename this file

var edgeMap = {};

var maxFlowBetweenAccounts = function (sourceAddr, targetAddr) {
  // sourceAddr: the hopeful sender's address
  // targetAddr: the potential recipient's address
  
  var query = 'MATCH (:User {address:{targetAddr}})' + 
    '-[limit:TRUST*..3]->(:User {address:{sourceAddr}}) ' + 
    'WHERE limit.amount > 0 RETURN limit';

  var paths = neoQuerySync(query, {
    sourceAddr: sourceAddr,
    targetAddr: targetAddr
  });
  // console.log('paths:', paths);
  var maxFlow = 0;

  // for each path
  paths.forEach(function(obj) {
    var path = obj.limit;

    // add edges to edgeMap
    path.forEach(function(edge) {
      var edgeId = edge._data.metadata.id;
      var limit = edge._data.data.limit;
      addToEdgeMap(edgeId, limit);
    });

    // get the path's capacity
    var pathCapacity = path.reduce(function(cap, edge) {
      var edgeId = edge._data.metadata.id;
      return edgeMap[edgeId].capacity < cap ? edgeMap[edgeId].capacity : cap;
    }, Number.POSITIVE_INFINITY);

    // subtract pathCapacity from each edge's capacity
    path.forEach(function(edge) {
      var edgeId = edge._data.metadata.id;
      edgeMap[edgeId].capacity -= pathCapacity;
    });

    // add capacity to flow
    maxFlow += pathCapacity;
  });

  resetEdgeMap();
  return maxFlow;
};

function addToEdgeMap(edgeId, limit) {
  // called on every edge
  // if edge exist
  // return
  // else
  // set edge to {limit:limit, capacity: limit}

  edgeMap[edgeId] = edgeMap[edgeId] || {
    limit: limit,
    capacity: limit
  }
}

function resetEdgeMap() {
  _.each(edgeMap, function(edge) {
    edge.capacity = edge.limit;
  });
}

// GLOBALS
helpers = {
  maxFlowBetweenAccounts: maxFlowBetweenAccounts
}