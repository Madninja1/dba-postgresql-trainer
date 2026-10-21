CREATE TABLE db_roles(role_name TEXT, can_login INTEGER, valid_until_seq INTEGER);
         INSERT INTO db_roles VALUES ('zeta',1,1),('alpha',1,1),('future',1,101),('nologin',0,0);
