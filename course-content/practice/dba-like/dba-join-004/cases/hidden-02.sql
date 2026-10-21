CREATE TABLE servers(id INTEGER PRIMARY KEY, hostname TEXT); CREATE TABLE databases(id INTEGER PRIMARY KEY, name TEXT, server_id INTEGER);
         INSERT INTO servers VALUES (1,'z'),(2,'a'); INSERT INTO databases VALUES (1,'zdb',2),(2,'adb',1);
