SELECT d.name AS database_name, s.state
FROM databases AS d
LEFT JOIN sessions AS s
  ON s.database_id = d.id
 AND s.state = 'active'
ORDER BY d.name ASC;
