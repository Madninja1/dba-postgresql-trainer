SELECT d.name AS database_name, r.role_name AS owner_role
       FROM databases AS d
       JOIN db_roles AS r ON r.id = d.owner_role_id
       WHERE r.can_login = 0
       ORDER BY d.name ASC;
