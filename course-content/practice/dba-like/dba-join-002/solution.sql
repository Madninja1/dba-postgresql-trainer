SELECT d.name AS database_name
FROM databases AS d
LEFT JOIN sessions AS s
  ON s.database_id = d.id
 AND s.state = 'active'
WHERE s.id IS NULL
ORDER BY d.name ASC;
