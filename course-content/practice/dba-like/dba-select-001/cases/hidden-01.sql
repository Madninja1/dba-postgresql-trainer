CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, role_name TEXT NOT NULL, state TEXT NOT NULL, environment TEXT NOT NULL, duration_seconds INTEGER NOT NULL);
INSERT INTO sessions VALUES
(1,'core','alice','active','prod',301),(2,'core','bob','idle','prod',999),(3,'warehouse','etl','active','prod',1200),(4,'sandbox','dev','active','stage',700),(5,'api','svc','active','prod',299);
