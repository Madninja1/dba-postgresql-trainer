CREATE TABLE query_stats (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, duration_ms INTEGER NOT NULL);
INSERT INTO query_stats VALUES (1,'appdb',100),(2,'appdb',200),(3,'appdb',300),(4,'audit',500),(5,'audit',400),(6,'billing',250),(7,'billing',250),(8,'billing',250),(9,'metrics',50),(10,'metrics',60),(11,'metrics',70);
