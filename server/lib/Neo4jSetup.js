// creates connection to Neo4j db
	// takes optional str of db URL 
		// (or uses env vars NEO4J_URL or GRAPHENEDB_URL)
this.N4JDB = new Meteor.Neo4j();

// GLOBAL
// wrappers for query functions
neoQuery = Meteor.N4JDB.query;
neoQuerySync = Async.wrap(Meteor.N4JDB.query);