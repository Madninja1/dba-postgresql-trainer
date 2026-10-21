CREATE TABLE servers(id INTEGER PRIMARY KEY, hostname TEXT); CREATE TABLE databases(id INTEGER PRIMARY KEY, name TEXT, server_id INTEGER);
         INSERT INTO servers VALUES (1,'pg01'),(2,'pg02'); INSERT INTO databases VALUES (1,'app',1),(2,'audit',2),(3,'billing',1);
