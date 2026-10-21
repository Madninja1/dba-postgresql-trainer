CREATE TABLE databases(id INTEGER PRIMARY KEY, name TEXT); CREATE TABLE backups(id INTEGER PRIMARY KEY, database_id INTEGER, status TEXT, size_mb INTEGER);
         INSERT INTO databases VALUES (1,'app'),(2,'audit'); INSERT INTO backups VALUES (1,1,'success',100),(2,1,'failed',0),(3,2,'success',50);
