CREATE TABLE incidents (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, severity TEXT NOT NULL, age_hours INTEGER NOT NULL, resolved_at TEXT);
INSERT INTO incidents VALUES
(20,'pg-q','critical',7,'done'),(21,'pg-q','high',7,'done'),(22,'pg-q','medium',7,NULL);
