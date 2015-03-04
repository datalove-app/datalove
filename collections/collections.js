Users = Meteor.neo4j.query('MATCH (users:User) RETURN users, count(users)');
