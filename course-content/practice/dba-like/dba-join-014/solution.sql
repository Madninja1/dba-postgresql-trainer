SELECT d.name AS database_name, s.hostname, r.role_name AS owner_role, b.finished_seq, b.size_mb
       FROM backups AS b
       JOIN databases AS d ON d.id = b.database_id
       JOIN servers AS s ON s.id = d.server_id
       JOIN db_roles AS r ON r.id = d.owner_role_id
       LEFT JOIN backups AS later
         ON later.database_id = b.database_id
        AND later.status = 'success'
        AND (later.finished_seq > b.finished_seq OR (later.finished_seq = b.finished_seq AND later.id > b.id))
       WHERE b.status = 'success' AND later.id IS NULL
       ORDER BY d.name ASC;
