CREATE TABLE backups (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, status TEXT NOT NULL, age_days INTEGER NOT NULL);
INSERT INTO backups VALUES (1,'pg-a','failed',1),(2,'pg-a','failed',4),(3,'pg-a','success',2),(4,'pg-b','failed',8),(5,'pg-b','failed',2),(6,'pg-c','failed',1),(7,'pg-c','failed',2),(8,'pg-c','failed',7);
