CREATE TABLE servers (id INTEGER PRIMARY KEY, hostname TEXT NOT NULL); CREATE TABLE db_roles (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE databases (id INTEGER PRIMARY KEY, server_id INTEGER NOT NULL, name TEXT NOT NULL, owner_role_id INTEGER NOT NULL);
INSERT INTO servers VALUES (1,'single'); INSERT INTO db_roles VALUES (1,'owner'); INSERT INTO databases VALUES (1,1,'onlydb',1);
