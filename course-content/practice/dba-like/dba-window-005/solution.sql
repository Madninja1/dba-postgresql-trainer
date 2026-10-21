SELECT backup_id, database_name, finished_seq, size_mb,
              SUM(size_mb) OVER (
                PARTITION BY database_name
                ORDER BY finished_seq ASC, backup_id ASC
                ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
              ) AS running_size_mb
       FROM backups
       WHERE status = 'success'
       ORDER BY database_name ASC, finished_seq ASC, backup_id ASC;
