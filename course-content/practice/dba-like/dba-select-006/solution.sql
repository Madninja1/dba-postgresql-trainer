SELECT database_name, job_id, error_code
       FROM backup_jobs
       WHERE status = 'failed'
       ORDER BY database_name ASC, job_id DESC;
