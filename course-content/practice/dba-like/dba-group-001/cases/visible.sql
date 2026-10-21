CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, state TEXT NOT NULL);
INSERT INTO sessions VALUES (1,'appdb','active'),(2,'appdb','active'),(3,'appdb','idle'),(4,'audit','active'),(5,'billing','active'),(6,'billing','active'),(7,'billing','active');
