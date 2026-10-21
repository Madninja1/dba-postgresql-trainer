CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, state TEXT NOT NULL);
INSERT INTO sessions VALUES (1,'core','active'),(2,'core','idle'),(3,'warehouse','active'),(4,'warehouse','active'),(5,'warehouse','idle'),(6,'metrics','active'),(7,'metrics','active');
