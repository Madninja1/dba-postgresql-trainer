SELECT d.name AS database_name
       FROM databases AS d
       LEFT JOIN backups AS b ON b.database_id = d.id
       WHERE b.id IS NULL
       ORDER BY d.name ASC;
