CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, state TEXT NOT NULL);
INSERT INTO sessions VALUES (1,'one','active'),(2,'two','idle'),(3,'three','active');
