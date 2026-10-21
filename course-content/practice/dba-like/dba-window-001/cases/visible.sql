CREATE TABLE query_stats (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, duration_ms INTEGER NOT NULL);
INSERT INTO query_stats VALUES (1,'appdb',100),(2,'appdb',500),(3,'appdb',500),(4,'appdb',50),(5,'audit',900),(6,'audit',100),(7,'billing',300);
