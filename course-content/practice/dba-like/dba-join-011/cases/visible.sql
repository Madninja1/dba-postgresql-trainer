CREATE TABLE db_roles(id INTEGER, role_name TEXT); CREATE TABLE databases(id INTEGER, name TEXT, owner_role_id INTEGER); CREATE TABLE query_stats(id INTEGER, database_id INTEGER, duration_ms INTEGER);
         INSERT INTO db_roles VALUES (1,'app_owner'),(2,'dba'); INSERT INTO databases VALUES (1,'app',1),(2,'audit',2); INSERT INTO query_stats VALUES (1,1,100),(2,1,800),(3,2,800),(4,2,500);
