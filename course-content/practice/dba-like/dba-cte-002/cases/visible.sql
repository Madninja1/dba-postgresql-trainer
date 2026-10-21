CREATE TABLE servers (id INTEGER PRIMARY KEY, hostname TEXT NOT NULL); CREATE TABLE sessions (id INTEGER PRIMARY KEY, server_id INTEGER NOT NULL, state TEXT NOT NULL);
INSERT INTO servers VALUES (1,'pg-a'),(2,'pg-b'),(3,'pg-c'),(4,'pg-d'); INSERT INTO sessions VALUES (1,1,'active'),(2,1,'active'),(3,1,'active'),(4,2,'active'),(5,2,'idle'),(6,3,'active');
