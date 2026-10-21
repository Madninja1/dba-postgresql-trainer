SELECT job_id, database_name, priority,
              CASE priority WHEN 'critical' THEN 1 WHEN 'high' THEN 2 WHEN 'normal' THEN 3 ELSE 4 END AS priority_rank
       FROM maintenance_jobs
       WHERE status = 'pending'
       ORDER BY priority_rank ASC, scheduled_seq ASC, job_id ASC;
