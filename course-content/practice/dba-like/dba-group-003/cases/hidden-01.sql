CREATE TABLE backups (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, status TEXT NOT NULL, age_days INTEGER NOT NULL);
INSERT INTO backups VALUES (1,'pg-x','failed',7),(2,'pg-x','failed',7),(3,'pg-y','failed',0),(4,'pg-y','failed',9),(5,'pg-z','success',1),(6,'pg-z','failed',1),(7,'pg-z','failed',2);
