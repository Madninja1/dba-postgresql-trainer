SELECT d.name AS database_name
       FROM databases AS d
       WHERE NOT EXISTS (
         SELECT 1 FROM sessions AS s
         WHERE s.database_id = d.id AND s.state = 'active'
       )
       ORDER BY d.name ASC;
