// collections.js
/*
	contains: 
		additional collections (e.g. Products) 
		Mongo-stored Neo4j collections
 */

neoDB = {
	// Meteor.neo4j.query returns a reactive Object 
		// contains all props saved to Neo4j 
  Users: Meteor.neo4j.query('MATCH (user:User) RETURN user, count(user)'),

  Limits: Meteor.neo4j.query('MATCH ()-[limit:TRUST]->() WHERE limit.amount > 0 RETURN limit')
};

// Products = new Meteor.Collection('products');
// Transactions = new Meteor.Collection('transactions');