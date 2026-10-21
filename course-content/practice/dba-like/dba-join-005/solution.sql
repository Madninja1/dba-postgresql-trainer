SELECT s.id AS session_id, r.role_name
       FROM sessions AS s
       JOIN db_roles AS r ON r.id = s.role_id
       ORDER BY s.id ASC;
