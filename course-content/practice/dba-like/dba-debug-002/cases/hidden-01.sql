CREATE TABLE databases (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE backups (id INTEGER PRIMARY KEY, database_id INTEGER NOT NULL, status TEXT NOT NULL, finished_seq INTEGER NOT NULL);
INSERT INTO databases VALUES (1,'core'),(2,'empty'); INSERT INTO backups VALUES (1,1,'success',100),(2,1,'success',100),(3,1,'success',90);
