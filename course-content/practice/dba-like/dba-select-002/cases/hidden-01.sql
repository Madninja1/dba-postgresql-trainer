CREATE TABLE databases (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, name TEXT NOT NULL, size_mb INTEGER NOT NULL, connection_usage_percent INTEGER NOT NULL, status TEXT NOT NULL);
INSERT INTO databases VALUES
(1,'pg-1','core',51199,80,'online'),(2,'pg-1','warehouse',51200,20,'online'),(3,'pg-2','legacy',99999,99,'offline'),(4,'pg-2','tiny',100,79,'online');
