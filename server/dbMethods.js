Meteor.neo4j.methods({
  /*
  define queries that our clients can run

  each prop is a function that returns the string of our queries
  ex: return 'MATCH (a:Player {_id:{playerId}}) DELETE a'
    > this will match a doc in Player based on an obj with a prop
      playerId

   */
});