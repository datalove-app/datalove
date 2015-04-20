// collections.js
/*
	contains: 
		additional collections (e.g. Products) 
		Mongo-stored Neo4j collections
 */

neo = {
	// Meteor.neo4j.query returns a reactive Object 
		// contains all props saved to Neo4j 
  Users: Meteor.neo4j.query('MATCH (users:User) RETURN users, count(users)'),

  Limits: Meteor.neo4j.query('MATCH ()-[limits:TRUST]->() RETURN limits')
};

Products = new Meteor.Collection('products');