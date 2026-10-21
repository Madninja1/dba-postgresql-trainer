CREATE TABLE database_health(database_name TEXT, environment TEXT, free_percent REAL, connections_used INTEGER, connections_limit INTEGER, read_only INTEGER);
         INSERT INTO database_health VALUES ('a','production',1,99,100,0),('b','production',50,100,100,0),('c','production',50,1,100,1),('stage','staging',0,100,100,1);
