CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, role_name TEXT NOT NULL, state TEXT NOT NULL, environment TEXT NOT NULL, duration_seconds INTEGER NOT NULL);
INSERT INTO sessions VALUES
(1,'appdb','app','active','prod',420),(2,'audit','report','idle','prod',900),(3,'billing','batch','active','prod',300),(4,'devdb','dev','active','dev',800),(5,'metrics','monitor','active','prod',120),(6,'archive','ops','active','prod',420);
