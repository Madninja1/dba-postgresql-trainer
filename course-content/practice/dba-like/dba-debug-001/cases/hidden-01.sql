CREATE TABLE databases (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_id INTEGER NOT NULL, state TEXT NOT NULL);
INSERT INTO databases VALUES (1,'core'),(2,'idle_only'),(3,'empty'); INSERT INTO sessions VALUES (1,1,'active'),(2,2,'idle'),(3,2,'idle');
