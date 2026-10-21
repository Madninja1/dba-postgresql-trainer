CREATE TABLE backup_jobs(job_id INTEGER, database_name TEXT, status TEXT, error_code TEXT);
         INSERT INTO backup_jobs VALUES (100,'x','failed','E1'),(101,'x','failed','E2'),(1,'a','failed',NULL),(3,'a','cancelled','E3');
