CREATE TABLE backup_jobs(job_id INTEGER, database_name TEXT, status TEXT, error_code TEXT);
         INSERT INTO backup_jobs VALUES (1,'audit','running',NULL),(2,'audit','failed','E_AUTH'),(3,'core','success',NULL);
