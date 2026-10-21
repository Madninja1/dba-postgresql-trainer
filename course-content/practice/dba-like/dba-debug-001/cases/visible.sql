CREATE TABLE databases (id INTEGER PRIMARY KEY, name TEXT NOT NULL); CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_id INTEGER NOT NULL, state TEXT NOT NULL);
INSERT INTO databases VALUES (1,'appdb'),(2,'audit'),(3,'billing'); INSERT INTO sessions VALUES (1,1,'active'),(2,2,'idle');
