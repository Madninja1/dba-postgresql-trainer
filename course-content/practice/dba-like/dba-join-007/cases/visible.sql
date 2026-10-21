CREATE TABLE db_roles(id INTEGER, role_name TEXT, can_login INTEGER); CREATE TABLE databases(id INTEGER, name TEXT); CREATE TABLE role_grants(role_id INTEGER, database_id INTEGER);
         INSERT INTO db_roles VALUES (1,'app',1),(2,'readonly',0),(3,'report',1); INSERT INTO databases VALUES (10,'appdb'),(20,'audit'); INSERT INTO role_grants VALUES (1,10),(1,20),(2,10),(3,20);
