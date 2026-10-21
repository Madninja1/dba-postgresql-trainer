SELECT s.hostname
       FROM servers AS s
       LEFT JOIN incidents AS i ON i.server_id = s.id AND i.resolved = 0
       WHERE s.environment = 'production' AND i.id IS NULL
       ORDER BY s.hostname ASC;
