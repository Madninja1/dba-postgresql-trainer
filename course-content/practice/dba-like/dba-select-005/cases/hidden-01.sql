CREATE TABLE replicas(replica_name TEXT, database_name TEXT, state TEXT, lag_seconds INTEGER);
         INSERT INTO replicas VALUES ('east','core','streaming',30),('west','core','stopped',500),('north','audit','streaming',31);
