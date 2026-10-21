CREATE TABLE database_health(database_name TEXT, environment TEXT, free_percent REAL, connections_used INTEGER, connections_limit INTEGER, read_only INTEGER);
         INSERT INTO database_health VALUES ('app','production',20,95,100,0),('audit','production',10,10,100,0),('billing','production',5,99,100,1),('dev','development',1,100,100,1),('ok','production',30,20,100,0);
