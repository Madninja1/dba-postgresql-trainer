SELECT d.name AS database_name, s.hostname, r.name AS owner_role
FROM databases AS d
JOIN servers AS s ON s.id = d.server_id
JOIN db_roles AS r ON r.id = d.owner_role_id
ORDER BY s.hostname ASC, d.name ASC;
