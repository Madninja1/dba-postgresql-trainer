CREATE TABLE servers(id INTEGER, hostname TEXT, environment TEXT); CREATE TABLE incidents(id INTEGER, server_id INTEGER, resolved INTEGER);
         INSERT INTO servers VALUES (1,'x','production'),(2,'y','development'),(3,'z','production'); INSERT INTO incidents VALUES (1,1,0),(2,2,0),(3,3,1);
