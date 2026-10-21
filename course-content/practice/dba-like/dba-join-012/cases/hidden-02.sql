CREATE TABLE db_roles(id INTEGER, role_name TEXT, can_login INTEGER); CREATE TABLE databases(id INTEGER, name TEXT, owner_role_id INTEGER);
         INSERT INTO db_roles VALUES (1,'u',1),(2,'g',0); INSERT INTO databases VALUES (1,'zdb',2),(2,'adb',2),(3,'ok',1);
