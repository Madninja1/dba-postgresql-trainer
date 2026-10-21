CREATE TABLE query_stats (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, duration_ms INTEGER NOT NULL);
INSERT INTO query_stats VALUES (1,'core',199),(2,'core',200),(3,'core',201),(4,'warehouse',100),(5,'warehouse',500),(6,'warehouse',300),(7,'single',1000);
