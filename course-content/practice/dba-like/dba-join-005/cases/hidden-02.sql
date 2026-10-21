CREATE TABLE db_roles(id INTEGER PRIMARY KEY, role_name TEXT); CREATE TABLE sessions(id INTEGER PRIMARY KEY, role_id INTEGER);
         INSERT INTO db_roles VALUES (1,'z'),(2,'a'); INSERT INTO sessions VALUES (9,2),(2,1);
