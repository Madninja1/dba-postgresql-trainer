CREATE TABLE backups (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, status TEXT NOT NULL, finished_seq INTEGER NOT NULL, size_mb INTEGER NOT NULL);
INSERT INTO backups VALUES (1,'a','success',1,100),(2,'a','failed',2,1000),(3,'a','success',3,150),(4,'b','success',1,500);
