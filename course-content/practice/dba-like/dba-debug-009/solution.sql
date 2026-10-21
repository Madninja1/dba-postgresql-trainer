WITH ranked AS (
         SELECT database_name, finished_seq, size_mb,
                ROW_NUMBER() OVER (
                  PARTITION BY database_name
                  ORDER BY finished_seq DESC, backup_id DESC
                ) AS rn
         FROM backups
         WHERE status = 'success'
       )
       SELECT database_name, finished_seq, size_mb
       FROM ranked
       WHERE rn = 1
       ORDER BY database_name ASC;
