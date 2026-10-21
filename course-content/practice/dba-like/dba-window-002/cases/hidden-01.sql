CREATE TABLE backups (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, status TEXT NOT NULL, finished_seq INTEGER NOT NULL, size_mb INTEGER NOT NULL);
INSERT INTO backups VALUES (10,'core','success',1,100),(11,'core','success',2,200),(12,'core','success',3,299),(13,'warehouse','success',1,1000),(14,'warehouse','success',2,1200);
