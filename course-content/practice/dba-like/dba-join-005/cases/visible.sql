CREATE TABLE db_roles(id INTEGER PRIMARY KEY, role_name TEXT); CREATE TABLE sessions(id INTEGER PRIMARY KEY, role_id INTEGER);
         INSERT INTO db_roles VALUES (1,'app'),(2,'admin'); INSERT INTO sessions VALUES (10,1),(11,2),(12,1);
