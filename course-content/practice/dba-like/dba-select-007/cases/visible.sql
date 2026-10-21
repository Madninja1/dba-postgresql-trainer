CREATE TABLE db_roles(role_name TEXT, can_login INTEGER, valid_until_seq INTEGER);
         INSERT INTO db_roles VALUES ('app',1,90),('admin',1,NULL),('etl',0,80),('report',1,120),('legacy',1,90);
