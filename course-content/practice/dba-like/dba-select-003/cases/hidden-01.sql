CREATE TABLE incidents (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, severity TEXT NOT NULL, age_hours INTEGER NOT NULL, resolved_at TEXT);
INSERT INTO incidents VALUES
(10,'pg-x','high',40,NULL),(11,'pg-y','critical',1,NULL),(12,'pg-z','high',40,NULL),(13,'pg-y','critical',1,NULL),(14,'pg-z','low',999,NULL);
