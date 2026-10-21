CREATE TABLE db_roles(id INTEGER, role_name TEXT); CREATE TABLE databases(id INTEGER, name TEXT, owner_role_id INTEGER); CREATE TABLE query_stats(id INTEGER, database_id INTEGER, duration_ms INTEGER);
         INSERT INTO db_roles VALUES (1,'a'),(2,'b'); INSERT INTO databases VALUES (1,'z',1),(2,'a',2); INSERT INTO query_stats VALUES (9,1,900),(2,2,900),(3,2,901);
