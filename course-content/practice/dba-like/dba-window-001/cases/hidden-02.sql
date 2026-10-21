CREATE TABLE query_stats (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, duration_ms INTEGER NOT NULL);
INSERT INTO query_stats VALUES (1,'single',42);
