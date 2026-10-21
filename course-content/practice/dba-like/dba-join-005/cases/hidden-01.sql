CREATE TABLE db_roles(id INTEGER PRIMARY KEY, role_name TEXT); CREATE TABLE sessions(id INTEGER PRIMARY KEY, role_id INTEGER);
         INSERT INTO db_roles VALUES (3,'report'); INSERT INTO sessions VALUES (1,3);
