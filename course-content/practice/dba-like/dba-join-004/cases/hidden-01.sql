CREATE TABLE servers(id INTEGER PRIMARY KEY, hostname TEXT); CREATE TABLE databases(id INTEGER PRIMARY KEY, name TEXT, server_id INTEGER);
         INSERT INTO servers VALUES (5,'node-a'); INSERT INTO databases VALUES (1,'core',5);
