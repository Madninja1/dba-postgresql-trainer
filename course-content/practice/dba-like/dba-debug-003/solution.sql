SELECT backup_id, database_name
       FROM backups
       WHERE finished_seq IS NULL
       ORDER BY backup_id ASC;
