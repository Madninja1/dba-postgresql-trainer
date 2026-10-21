CREATE TABLE incidents (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, severity TEXT NOT NULL, age_hours INTEGER NOT NULL, resolved_at TEXT);
INSERT INTO incidents VALUES
(1,'pg-a','high',5,NULL),(2,'pg-b','critical',2,NULL),(3,'pg-c','medium',100,NULL),(4,'pg-a','critical',9,'2026-10-01'),(5,'pg-d','critical',12,NULL),(6,'pg-b','high',12,NULL);
