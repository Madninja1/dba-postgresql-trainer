SELECT r.role_name, COUNT(DISTINCT s.id) AS server_count
       FROM role_grants AS g
       JOIN db_roles AS r ON r.id = g.role_id
       JOIN databases AS d ON d.id = g.database_id
       JOIN servers AS s ON s.id = d.server_id
       GROUP BY r.id, r.role_name
       HAVING COUNT(DISTINCT s.id) >= 2
       ORDER BY server_count DESC, r.role_name ASC;
