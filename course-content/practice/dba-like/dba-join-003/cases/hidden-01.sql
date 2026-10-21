CREATE TABLE databases (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE backups (id INTEGER PRIMARY KEY, database_id INTEGER NOT NULL, status TEXT NOT NULL, finished_seq INTEGER NOT NULL);
INSERT INTO databases VALUES (1,'core'),(2,'warehouse'); INSERT INTO backups VALUES (10,1,'success',100),(11,1,'success',100),(12,1,'success',90),(20,2,'failed',200);
