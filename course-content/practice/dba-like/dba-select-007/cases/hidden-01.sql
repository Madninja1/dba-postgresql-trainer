CREATE TABLE db_roles(role_name TEXT, can_login INTEGER, valid_until_seq INTEGER);
         INSERT INTO db_roles VALUES ('alice',1,99),('bob',1,100),('group_ro',0,50),('carol',1,NULL);
