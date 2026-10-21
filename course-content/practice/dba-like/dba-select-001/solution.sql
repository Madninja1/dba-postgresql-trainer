SELECT database_name, role_name, duration_seconds
FROM sessions
WHERE state = 'active'
  AND environment = 'prod'
  AND duration_seconds >= 300
ORDER BY duration_seconds DESC, database_name ASC;
