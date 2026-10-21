SELECT d.name AS database_name, s.hostname
       FROM databases AS d
       JOIN servers AS s ON s.id = d.server_id
       ORDER BY d.name ASC;
