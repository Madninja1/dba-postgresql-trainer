CREATE TABLE database_health(database_name TEXT, environment TEXT, free_percent REAL, connections_used INTEGER, connections_limit INTEGER, read_only INTEGER);
         INSERT INTO database_health VALUES ('ro','production',40,1,100,1),('disk','production',14.9,89,100,0),('conn','production',15,90,100,0),('zero','production',5,0,0,0);
