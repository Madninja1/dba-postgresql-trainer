CREATE TABLE sessions (id INTEGER PRIMARY KEY, database_name TEXT NOT NULL, role_name TEXT NOT NULL, state TEXT NOT NULL, environment TEXT NOT NULL, duration_seconds INTEGER NOT NULL);
INSERT INTO sessions VALUES
(1,'zeta','r1','active','prod',600),(2,'alpha','r2','active','prod',600),(3,'beta','r3','active','prod',300),(4,'gamma','r4','idle','prod',700);
