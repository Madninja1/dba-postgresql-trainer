SELECT b.id AS backup_id, d.name AS database_name, b.size_mb
       FROM backups AS b
       JOIN databases AS d ON d.id = b.database_id
       WHERE b.status = 'success'
       ORDER BY b.id ASC;
