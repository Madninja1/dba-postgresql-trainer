CREATE TABLE databases (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_id INTEGER NOT NULL, state TEXT NOT NULL);
INSERT INTO databases VALUES (1,'core'),(2,'empty'),(3,'idle_only'); INSERT INTO sessions VALUES (1,1,'active'),(2,1,'active'),(3,3,'idle');
