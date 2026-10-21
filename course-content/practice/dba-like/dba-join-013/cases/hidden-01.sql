CREATE TABLE servers(id INTEGER, hostname TEXT, environment TEXT); CREATE TABLE incidents(id INTEGER, server_id INTEGER, resolved INTEGER);
         INSERT INTO servers VALUES (1,'a','production'),(2,'b','production'); INSERT INTO incidents VALUES (1,1,1),(2,1,1);
