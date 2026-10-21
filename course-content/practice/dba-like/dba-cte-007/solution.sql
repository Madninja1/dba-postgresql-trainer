SELECT s.hostname
       FROM servers AS s
       WHERE EXISTS (SELECT 1 FROM databases AS d0 WHERE d0.server_id = s.id)
         AND NOT EXISTS (
           SELECT 1
           FROM databases AS d
           WHERE d.server_id = s.id
             AND NOT EXISTS (
               SELECT 1
               FROM backups AS b
               WHERE b.database_id = d.id AND b.status = 'success'
             )
         )
       ORDER BY s.hostname ASC;
