CREATE TABLE database_health(database_name TEXT, environment TEXT, size_gb INTEGER, accepting_connections INTEGER);
         INSERT INTO database_health VALUES ('alpha','production',55,0),('beta','production',56,0),('gamma','production',55,0),('lab','development',999,0);
