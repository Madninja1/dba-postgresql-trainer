CREATE TABLE databases(id INTEGER PRIMARY KEY, name TEXT); CREATE TABLE backups(id INTEGER PRIMARY KEY, database_id INTEGER, status TEXT, size_mb INTEGER);
         INSERT INTO databases VALUES (9,'core'); INSERT INTO backups VALUES (5,9,'success',1),(6,9,'cancelled',2);
