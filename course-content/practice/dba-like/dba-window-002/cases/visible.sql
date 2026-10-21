CREATE TABLE backups (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, status TEXT NOT NULL, finished_seq INTEGER NOT NULL, size_mb INTEGER NOT NULL);
INSERT INTO backups VALUES (1,'appdb','success',10,1000),(2,'appdb','success',20,1080),(3,'appdb','success',30,1250),(4,'audit','success',5,500),(5,'audit','failed',10,900),(6,'audit','success',15,700);
