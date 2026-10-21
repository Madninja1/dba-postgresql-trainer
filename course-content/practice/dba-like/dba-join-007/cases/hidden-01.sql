CREATE TABLE db_roles(id INTEGER, role_name TEXT, can_login INTEGER); CREATE TABLE databases(id INTEGER, name TEXT); CREATE TABLE role_grants(role_id INTEGER, database_id INTEGER);
         INSERT INTO db_roles VALUES (1,'alice',1),(2,'group',0); INSERT INTO databases VALUES (1,'core'); INSERT INTO role_grants VALUES (1,1),(2,1);
