CREATE TABLE query_stats (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, duration_ms INTEGER NOT NULL);
INSERT INTO query_stats VALUES (10,'core',10),(11,'core',20),(12,'core',30),(13,'warehouse',5),(14,'warehouse',5),(15,'warehouse',5);
