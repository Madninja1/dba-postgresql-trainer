CREATE TABLE maintenance_jobs(job_id INTEGER, database_name TEXT, priority TEXT, status TEXT, scheduled_seq INTEGER);
         INSERT INTO maintenance_jobs VALUES (7,'a','critical','pending',10),(6,'b','critical','pending',10),(8,'c','unknown','pending',1),(9,'d','normal','done',0);
