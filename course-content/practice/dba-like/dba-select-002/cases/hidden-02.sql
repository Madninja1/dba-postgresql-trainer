CREATE TABLE databases (id INTEGER PRIMARY KEY, server_name TEXT NOT NULL, name TEXT NOT NULL, size_mb INTEGER NOT NULL, connection_usage_percent INTEGER NOT NULL, status TEXT NOT NULL);
INSERT INTO databases VALUES
(1,'pg-x','zdb',70000,88,'online'),(2,'pg-x','adb',70000,88,'online'),(3,'pg-y','bdb',1000,100,'online');
