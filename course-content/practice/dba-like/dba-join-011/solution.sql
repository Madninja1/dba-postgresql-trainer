SELECT q.id AS sample_id, d.name AS database_name, r.role_name AS owner_role, q.duration_ms
       FROM query_stats AS q
       JOIN databases AS d ON d.id = q.database_id
       JOIN db_roles AS r ON r.id = d.owner_role_id
       WHERE q.duration_ms >= 500
       ORDER BY q.duration_ms DESC, q.id ASC;
