CREATE TABLE replicas(replica_name TEXT, database_name TEXT, state TEXT, lag_seconds INTEGER);
         INSERT INTO replicas VALUES ('r1','app','streaming',12),('r2','app','streaming',45),('r3','billing','catching_up',120),('r4','billing','streaming',45);
