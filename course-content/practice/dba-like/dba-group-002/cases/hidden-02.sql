CREATE TABLE query_stats (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, duration_ms INTEGER NOT NULL);
INSERT INTO query_stats VALUES (1,'a',200),(2,'a',200),(3,'a',200),(4,'b',300),(5,'b',300),(6,'b',300),(7,'c',199),(8,'c',199),(9,'c',199);
