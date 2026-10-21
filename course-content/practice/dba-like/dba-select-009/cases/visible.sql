CREATE TABLE sessions(session_id INTEGER, database_name TEXT, state TEXT, duration_sec INTEGER, wait_event TEXT);
         INSERT INTO sessions VALUES (1,'app','active',120,'Lock'),(2,'app','active',180,NULL),(3,'billing','idle',500,'ClientRead'),(4,'billing','active',60,'IO');
