CREATE TABLE database_health(database_name TEXT, environment TEXT, size_gb INTEGER, accepting_connections INTEGER);
         INSERT INTO database_health VALUES ('catalog','production',15,0),('core','production',70,1),('stage','staging',90,0);
