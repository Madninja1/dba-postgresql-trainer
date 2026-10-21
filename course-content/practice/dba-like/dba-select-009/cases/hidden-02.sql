CREATE TABLE sessions(session_id INTEGER, database_name TEXT, state TEXT, duration_sec INTEGER, wait_event TEXT);
         INSERT INTO sessions VALUES (8,'x','active',90,'IO'),(9,'y','active',90,'Lock'),(10,'z','idle',900,'Lock'),(11,'q','active',300,'');
