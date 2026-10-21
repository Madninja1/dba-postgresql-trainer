CREATE TABLE databases(id INTEGER PRIMARY KEY, name TEXT); CREATE TABLE backups(id INTEGER PRIMARY KEY, database_id INTEGER, status TEXT, size_mb INTEGER);
         INSERT INTO databases VALUES (1,'x'),(2,'y'); INSERT INTO backups VALUES (10,2,'success',200),(2,1,'success',300);
