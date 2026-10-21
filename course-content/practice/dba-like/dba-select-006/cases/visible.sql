CREATE TABLE backup_jobs(job_id INTEGER, database_name TEXT, status TEXT, error_code TEXT);
         INSERT INTO backup_jobs VALUES (10,'app','failed','E_IO'),(11,'app','success',NULL),(12,'app','failed','E_NET'),(5,'billing','failed','E_SPACE');
