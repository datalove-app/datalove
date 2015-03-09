var edgeMap = {};

maxFlowBetweenAccounts = function (sourceAddr, targetAddr) {
  var query = 'MATCH (:User {key:"' + sourceAddr +
    '"})-[limit:TRUST*..3]->' +
    '(:User {key:"' + targetAddr +
    '"}) RETURN limit';

  var paths = neoQuerySync(query, null);
  console.log(paths);
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
