CREATE TABLE maintenance_jobs(job_id INTEGER, database_name TEXT, priority TEXT, status TEXT, scheduled_seq INTEGER);
         INSERT INTO maintenance_jobs VALUES (11,'x','high','pending',99),(12,'y','normal','pending',1),(13,'z','low','cancelled',1),(14,'q','emergency','pending',0);
