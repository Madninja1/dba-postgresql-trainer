CREATE TABLE servers(id INTEGER, hostname TEXT, environment TEXT); CREATE TABLE incidents(id INTEGER, server_id INTEGER, severity TEXT, resolved INTEGER, opened_seq INTEGER);
         INSERT INTO servers VALUES (1,'pg01','production'),(2,'pg02','staging'); INSERT INTO incidents VALUES (1,1,'critical',0,10),(2,1,'warning',1,5),(3,2,'critical',0,1);
