SELECT d.name AS database_name
       FROM databases AS d
       WHERE NOT EXISTS (
         SELECT 1
         FROM backups AS b
         WHERE b.database_id = d.id AND b.status = 'success'
       )
       ORDER BY d.name ASC;
