CREATE TABLE sessions(session_id INTEGER, database_name TEXT, state TEXT, duration_sec INTEGER, wait_event TEXT);
         INSERT INTO sessions VALUES (5,'core','active',59,'Lock'),(6,'core','active',60,NULL),(7,'audit','active',61,'Lock');
