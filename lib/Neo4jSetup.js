// allows client query execution
Meteor.neo4j.allowClientQuery = true;
// disables writing actions on the client
Meteor.neo4j.set.deny(neo4j.rules.write);