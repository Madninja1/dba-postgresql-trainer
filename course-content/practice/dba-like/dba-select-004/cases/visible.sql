CREATE TABLE database_health(database_name TEXT, environment TEXT, size_gb INTEGER, accepting_connections INTEGER);
         INSERT INTO database_health VALUES ('app','production',120,0),('audit','production',40,1),('billing','production',120,0),('dev','development',300,0);
