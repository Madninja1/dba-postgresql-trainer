CREATE TABLE db_roles(id INTEGER, role_name TEXT, can_login INTEGER); CREATE TABLE databases(id INTEGER, name TEXT, owner_role_id INTEGER);
         INSERT INTO db_roles VALUES (1,'app_owner',0),(2,'alice',1),(3,'report_owner',0); INSERT INTO databases VALUES (1,'app',1),(2,'audit',2),(3,'report',3);
