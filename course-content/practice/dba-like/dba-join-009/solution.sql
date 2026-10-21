SELECT s.id AS session_id, d.name AS database_name, srv.hostname, r.role_name
       FROM sessions AS s
       JOIN databases AS d ON d.id = s.database_id
       JOIN servers AS srv ON srv.id = d.server_id
       JOIN db_roles AS r ON r.id = s.role_id
       WHERE s.state = 'active'
       ORDER BY d.name ASC, s.id ASC;
