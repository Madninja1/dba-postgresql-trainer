CREATE TABLE maintenance_jobs(job_id INTEGER, database_name TEXT, priority TEXT, status TEXT, scheduled_seq INTEGER);
         INSERT INTO maintenance_jobs VALUES (1,'app','normal','pending',20),(2,'billing','critical','pending',30),(3,'audit','high','running',1),(4,'core','high','pending',10),(5,'misc','low','pending',5);
