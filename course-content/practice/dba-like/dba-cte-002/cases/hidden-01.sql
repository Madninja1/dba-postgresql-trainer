CREATE TABLE servers (id INTEGER PRIMARY KEY, hostname TEXT NOT NULL); CREATE TABLE sessions (id INTEGER PRIMARY KEY, server_id INTEGER NOT NULL, state TEXT NOT NULL);
INSERT INTO servers VALUES (1,'s1'),(2,'s2'),(3,'s3'); INSERT INTO sessions VALUES (1,1,'active'),(2,1,'active'),(3,2,'idle');
