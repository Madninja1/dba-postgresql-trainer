CREATE TABLE databases (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, name TEXT NOT NULL, size_mb INTEGER NOT NULL);
INSERT INTO databases VALUES (1,'pg-a','appdb',100),(2,'pg-a','audit',200),(3,'pg-a','billing',600),(4,'pg-b','core',300),(5,'pg-b','metrics',300),(6,'pg-c','only',999);
