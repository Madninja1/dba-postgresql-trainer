CREATE TABLE servers(id INTEGER, hostname TEXT, environment TEXT); CREATE TABLE incidents(id INTEGER, server_id INTEGER, severity TEXT, resolved INTEGER, opened_seq INTEGER);
         INSERT INTO servers VALUES (1,'x','production'),(2,'y','development'); INSERT INTO incidents VALUES (10,1,'info',0,100),(11,2,'critical',0,1),(12,1,'critical',1,2);
