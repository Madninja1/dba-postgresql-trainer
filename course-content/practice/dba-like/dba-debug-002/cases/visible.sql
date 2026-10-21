CREATE TABLE databases (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE backups (id INTEGER PRIMARY KEY, database_id INTEGER NOT NULL, status TEXT NOT NULL, finished_seq INTEGER NOT NULL);
INSERT INTO databases VALUES (1,'appdb'),(2,'audit'),(3,'billing'); INSERT INTO backups VALUES (1,1,'success',10),(2,1,'success',20),(3,1,'failed',30),(4,2,'failed',40),(5,3,'success',5);
