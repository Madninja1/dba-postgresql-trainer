SELECT r.role_name, d.name AS database_name
       FROM role_grants AS g
       JOIN db_roles AS r ON r.id = g.role_id
       JOIN databases AS d ON d.id = g.database_id
       WHERE r.can_login = 1
       ORDER BY r.role_name ASC, d.name ASC;
