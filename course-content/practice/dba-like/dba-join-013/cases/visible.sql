CREATE TABLE servers(id INTEGER, hostname TEXT, environment TEXT); CREATE TABLE incidents(id INTEGER, server_id INTEGER, resolved INTEGER);
         INSERT INTO servers VALUES (1,'pg01','production'),(2,'pg02','production'),(3,'pg03','staging'),(4,'pg04','production'); INSERT INTO incidents VALUES (1,1,1),(2,2,0),(3,3,0);
