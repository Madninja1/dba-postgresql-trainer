CREATE TABLE replicas(replica_name TEXT, database_name TEXT, state TEXT, lag_seconds INTEGER);
         INSERT INTO replicas VALUES ('a','db1','streaming',99),('b','db2','streaming',29),('c','db3','streaming',99),('d','db4','catching_up',100);
