CREATE TABLE databases (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, name TEXT NOT NULL, size_mb INTEGER NOT NULL, connection_usage_percent INTEGER NOT NULL, status TEXT NOT NULL);
INSERT INTO databases VALUES
(1,'pg-a','appdb',60000,55,'online'),(2,'pg-a','audit',12000,85,'online'),(3,'pg-b','billing',52000,90,'online'),(4,'pg-b','archive',90000,95,'offline'),(5,'pg-c','metrics',8000,40,'online');
