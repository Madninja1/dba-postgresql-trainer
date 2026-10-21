CREATE TABLE servers(id INTEGER, hostname TEXT, environment TEXT); CREATE TABLE incidents(id INTEGER, server_id INTEGER, severity TEXT, resolved INTEGER, opened_seq INTEGER);
         INSERT INTO servers VALUES (1,'a','production'); INSERT INTO incidents VALUES (2,1,'warning',0,5),(1,1,'critical',0,5);
