CREATE TABLE db_roles(id INTEGER, role_name TEXT, can_login INTEGER); CREATE TABLE databases(id INTEGER, name TEXT, owner_role_id INTEGER);
         INSERT INTO db_roles VALUES (1,'group',0); INSERT INTO databases VALUES (1,'x',1);
