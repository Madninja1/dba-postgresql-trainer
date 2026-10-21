CREATE TABLE backups (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, status TEXT NOT NULL, age_days INTEGER NOT NULL);
INSERT INTO backups VALUES (1,'solo','failed',1),(2,'old','failed',10),(3,'old','failed',11);
