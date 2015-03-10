// NEO4J Collections
neo = {
  Users: Meteor.neo4j.query('MATCH (users:User) RETURN users, count(users)'),

  Limits: Meteor.neo4j.query('MATCH ()-[limits:TRUST]->() RETURN limits')
};

Products = new Meteor.Collection('products');