CREATE TABLE db_roles(id INTEGER, role_name TEXT); CREATE TABLE databases(id INTEGER, name TEXT, owner_role_id INTEGER); CREATE TABLE query_stats(id INTEGER, database_id INTEGER, duration_ms INTEGER);
         INSERT INTO db_roles VALUES (1,'o'); INSERT INTO databases VALUES (1,'x',1); INSERT INTO query_stats VALUES (5,1,499),(6,1,500);
