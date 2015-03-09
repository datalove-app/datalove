this.N4JDB = new Meteor.Neo4j();

neoQuery = Meteor.N4JDB.query;
neoQuerySync = Async.wrap(Meteor.N4JDB.query);