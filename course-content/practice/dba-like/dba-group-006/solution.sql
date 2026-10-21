SELECT database_name, COUNT(*) AS failed_count
       FROM backups
       WHERE status = 'failed'
       GROUP BY database_name
       HAVING COUNT(*) >= 2
       ORDER BY failed_count DESC, database_name ASC;
